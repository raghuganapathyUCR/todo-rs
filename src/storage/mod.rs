use anyhow::Result;
use serde_json;
use std::fs;
use std::path::PathBuf;

use crate::domain::TodoList;

pub struct Storage {
    file_path: PathBuf,
}

impl Storage {
    pub fn new() -> Result<Self> {
        let mut path = dirs::data_dir().ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?;
        path.push("todo-rs");
        fs::create_dir_all(&path)?;
        path.push("todos.json");
        
        Ok(Self { file_path: path })
    }

    pub fn save(&self, todo_list: &TodoList) -> Result<()> {
        let json = serde_json::to_string_pretty(todo_list)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<TodoList> {
        if !self.file_path.exists() {
            return Ok(TodoList::new("Default".to_string()));
        }
        
        let contents = fs::read_to_string(&self.file_path)?;
        let todo_list = serde_json::from_str(&contents)?;
        Ok(todo_list)
    }
}
