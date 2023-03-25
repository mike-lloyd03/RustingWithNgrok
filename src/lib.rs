use askama::Template;
use std::fmt::Display;
use std::sync::Mutex;
use tokio::sync::broadcast;

/// The state of the application.
///
/// - `chat_messages` is a Vec that stores the chat history
/// - `tx` is the sender for a Tokio broadcast channel
pub struct AppState {
    chat_history: Mutex<Vec<ChatMsg>>,
    pub tx: broadcast::Sender<String>,
}

impl AppState {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let chat_history = Mutex::new(Vec::new());
        let (tx, _rx) = broadcast::channel(100);
        Self { chat_history, tx }
    }

    pub fn get_history(&self) -> Vec<ChatMsg> {
        self.chat_history.lock().unwrap().clone()
    }

    /// Pushes a ChatMsg onto the history and truncates the history list to 100 msgs
    pub fn push_history(&self, msg: ChatMsg) {
        let mut history = self.chat_history.lock().unwrap();
        history.push(msg);
        history.reverse();
        history.truncate(100);
        history.reverse();
    }
}

/// A message received from a client and sent to all other connected clients
#[derive(Debug, Clone)]
pub struct ChatMsg {
    pub username: String,
    pub text: String,
}

impl Display for ChatMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.username, self.text)
    }
}

#[derive(Template)]
#[template(path = "./index.html")]
pub struct IndexTemplate {
    pub chat_messages: Vec<ChatMsg>,
}
