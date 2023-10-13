use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Todo {
    id: u32,
    content: String,
    done: bool,
}

impl Todo {
    pub fn new(content: &str) -> Todo {
        Todo {
            id: rand::thread_rng().gen(),
            content: content.to_string(),
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

    pub fn set_done(&mut self, done: bool) {
        self.done = done;
    }
}
