use crate::{
    client::Client,
    messages::{ChatMsg, ConnectMsg, DisconnectMsg},
};
use actix::{Actor, Addr, Context, Handler};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub struct Server {
    pub clients: HashMap<usize, Addr<Client>>,
    pub chat_messages: Arc<Mutex<Vec<ChatMsg>>>,
}

impl Server {
    pub fn new(chat_messages: Arc<Mutex<Vec<ChatMsg>>>) -> Self {
        Server {
            clients: HashMap::new(),
            chat_messages,
        }
    }
}

impl Actor for Server {
    type Context = Context<Self>;
}

impl Handler<ConnectMsg> for Server {
    type Result = ();

    fn handle(&mut self, msg: ConnectMsg, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("{} connected", msg.username);
        self.clients.insert(msg.id, msg.addr);
    }
}

impl Handler<DisconnectMsg> for Server {
    type Result = ();

    fn handle(&mut self, msg: DisconnectMsg, _ctx: &mut Self::Context) -> Self::Result {
        log::info!("{} disconnected", msg.username);
        self.clients.remove(&msg.id);
    }
}

impl Handler<ChatMsg> for Server {
    type Result = ();

    fn handle(&mut self, msg: ChatMsg, _ctx: &mut Self::Context) {
        let mut chat_messages = self.chat_messages.lock().unwrap();
        chat_messages.push(msg.clone());
        chat_messages.reverse();
        chat_messages.truncate(100);
        chat_messages.reverse();

        for client in self.clients.values() {
            client.do_send(msg.clone())
        }
    }
}
