use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Conversation {
    pub messages: Vec<Message>
}

impl Conversation {
    pub fn new() -> Conversation {
        Conversation { messages: Vec::new() }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub user: bool,    // true if msg comes from user
    pub text: String,
}
