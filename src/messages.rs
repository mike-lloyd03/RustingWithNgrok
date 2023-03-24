use actix::{Addr, Message};
use std::fmt::Display;

use crate::client::Client;

#[derive(Debug, Clone, Message)]
#[rtype(result = "()")]
pub struct ChatMsg {
    pub username: String,
    pub text: String,
}

impl Display for ChatMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.username, self.text)
    }
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct ConnectMsg {
    pub id: usize,
    pub username: String,
    pub addr: Addr<Client>,
}

#[derive(Debug, Message)]
#[rtype(result = "()")]
pub struct DisconnectMsg {
    pub id: usize,
    pub username: String,
}
