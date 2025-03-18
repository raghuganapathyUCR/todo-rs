use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoList {
    pub name: String,
    pub todos: Vec<Todo>,
}

impl Todo {
    pub fn new(id: usize, title: String) -> Self {
        Self {
            id,
            title,
            description: None,
            completed: false,
        }
    }
}

impl TodoList {
    pub fn new(name: String) -> Self {
        Self {
            name,
            todos: Vec::new(),
        }
    }

    pub fn add_todo(&mut self, title: String) {
        let id = self.todos.len();
        self.todos.push(Todo::new(id, title));
    }
}
