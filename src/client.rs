use crate::{
    messages::{ChatMsg, ConnectMsg, DisconnectMsg},
    server::Server,
};
use actix::{
    fut, Actor, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler, Running,
    StreamHandler, WrapFuture,
};
use actix_web_actors::ws;

pub struct Client {
    pub id: usize,
    pub username: String,
    pub addr: Addr<Server>,
}

impl Actor for Client {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        self.addr
            .send(ConnectMsg {
                id: self.id,
                username: self.username.clone(),
                addr,
            })
            .into_actor(self)
            .then(|_, _, _| fut::ready(()))
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.addr.do_send(DisconnectMsg {
            id: self.id,
            username: self.username.clone(),
        });
        Running::Stop
    }
}

impl Handler<ChatMsg> for Client {
    type Result = ();

    fn handle(&mut self, msg: ChatMsg, ctx: &mut Self::Context) {
        ctx.text(msg.to_string());
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Client {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            let msg = ChatMsg {
                username: self.username.clone(),
                text: text.to_string(),
            };

            self.addr.do_send(msg);
        }
    }
}
