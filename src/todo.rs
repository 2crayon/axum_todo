use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    id: u32,
    content: String,
    done: bool,
}

impl Todo {
    pub fn new(content: String) -> Todo {
        Todo {
            id: rand::thread_rng().gen(),
            content,
            done: false,
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn done(&self) -> bool {
        self.done
    }
}
