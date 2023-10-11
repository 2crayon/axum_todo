use crate::todo::Todo;
use serde::{Deserialize, Serialize};
use std::{fs, io};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Database {
    todos: Vec<Todo>,
}

impl Database {
    pub fn new() -> Database {
        Database { todos: Vec::new() }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn get(&self, id: u32) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id() == id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut Todo> {
        self.todos.iter_mut().find(|todo| todo.id() == id)
    }

    pub fn remove(&mut self, id: u32) {
        self.todos.retain(|todo| todo.id() != id);
    }

    pub fn all(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn save(&self) -> Result<(), io::Error> {
        let data = serde_json::to_string(&self)?;
        fs::write("db.json", data)?;
        Ok(())
    }
}
