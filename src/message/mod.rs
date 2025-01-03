use serde::{Deserialize, Serialize};

mod handle;

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub is_at: bool,
}

pub type MessageHandler = fn(msg: Message);

pub fn default_message_handler(_msg: Message) {}
