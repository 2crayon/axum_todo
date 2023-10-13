use crate::todo::Todo;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Database {
    todos: Vec<Todo>,
}

impl Database {
    pub fn new() -> Database {
        Database::default()
    }

    pub fn todos(&self) -> &Vec<Todo> {
        &self.todos
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn delete(&mut self, id: u32) -> Option<Todo> {
        let index = self.todos.iter().position(|todo| todo.id() == id)?;
        Some(self.todos.remove(index))
    }

    pub fn toggle(&mut self, id: u32) -> Option<&Todo> {
        let todo = self.todos.iter_mut().find(|todo| todo.id() == id)?;
        todo.set_done(!todo.done());
        Some(todo)
    }

    pub fn clear_done(&mut self) {
        self.todos.retain(|todo| !todo.done());
    }

    pub fn save(&self) {
        let data = serde_json::to_string(&self).unwrap();
        fs::write("db.json", data).unwrap();
    }

    pub fn load() -> Database {
        match fs::read_to_string("db.json") {
            Ok(data) => match serde_json::from_str(&data) {
                Ok(db) => db,
                Err(_) => Database::new(),
            },
            Err(_) => Database::new(),
        }
    }
}
