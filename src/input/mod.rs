use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum Action {
    Quit,
    NewTodo,
    ToggleTodo,
    DeleteTodo,
    MoveTodoUp,
    MoveTodoDown,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct KeyBinding {
    pub code: String,
    pub modifiers: Vec<String>,
}

impl KeyBinding {
    pub fn matches(&self, key: &KeyEvent) -> bool {
        let code_matches = match key.code {
            KeyCode::Char(' ') => self.code == "space",
            KeyCode::Char(c) => self.code == c.to_string(),
            KeyCode::Esc => self.code == "esc",
            KeyCode::Enter => self.code == "enter",
            KeyCode::Up => self.code == "up",
            KeyCode::Down => self.code == "down",
            _ => false,
        };

        if !code_matches {
            return false;
        }

        let current_mods: Vec<String> = vec![
            (key.modifiers.contains(KeyModifiers::CONTROL), "ctrl"),
            (key.modifiers.contains(KeyModifiers::ALT), "alt"),
            (key.modifiers.contains(KeyModifiers::SHIFT), "shift"),
        ]
        .into_iter()
        .filter(|(active, _)| *active)
        .map(|(_, name)| name.to_string())
        .collect();

        self.modifiers.len() == current_mods.len()
            && self.modifiers.iter().all(|m| current_mods.contains(m))
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyBindings {
    bindings: HashMap<Action, KeyBinding>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let mut bindings = HashMap::new();
        
        // Default vim-like bindings
        bindings.insert(
            Action::Quit,
            KeyBinding {
                code: "q".to_string(),
                modifiers: vec![],
            },
        );
        bindings.insert(
            Action::NewTodo,
            KeyBinding {
                code: "n".to_string(),
                modifiers: vec![],
            },
        );
        bindings.insert(
            Action::ToggleTodo,
            KeyBinding {
                code: "space".to_string(),
                modifiers: vec![],
            },
        );
        bindings.insert(
            Action::DeleteTodo,
            KeyBinding {
                code: "d".to_string(),
                modifiers: vec![],
            },
        );
        bindings.insert(
            Action::MoveTodoUp,
            KeyBinding {
                code: "k".to_string(),
                modifiers: vec![],
            },
        );
        bindings.insert(
            Action::MoveTodoDown,
            KeyBinding {
                code: "j".to_string(),
                modifiers: vec![],
            },
        );

        Self { bindings }
    }
}

impl KeyBindings {
    pub fn load() -> Result<Self> {
        let mut config_path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        config_path.push("todo-rs");
        fs::create_dir_all(&config_path)?;
        config_path.push("keybindings.json");

        if !config_path.exists() {
            let default_bindings = Self::default();
            default_bindings.save()?;
            return Ok(default_bindings);
        }

        let contents = fs::read_to_string(&config_path)?;
        let bindings = serde_json::from_str(&contents)?;
        Ok(bindings)
    }

    pub fn save(&self) -> Result<()> {
        let mut config_path = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        config_path.push("todo-rs");
        fs::create_dir_all(&config_path)?;
        config_path.push("keybindings.json");

        let json = serde_json::to_string_pretty(self)?;
        fs::write(config_path, json)?;
        Ok(())
    }

    pub fn get_action(&self, key: &KeyEvent) -> Option<Action> {
        self.bindings
            .iter()
            .find(|(_, binding)| binding.matches(key))
            .map(|(action, _)| action.clone())
    }
}
