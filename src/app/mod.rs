use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Stdout};

use crate::domain::TodoList;
use crate::storage::Storage;
use crate::input::{KeyBindings, Action};
use crate::ui;

pub struct App {
    running: bool,
    todo_list: TodoList,
    storage: Storage,
    keybindings: KeyBindings,
    selected_index: usize,
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl App {
    pub fn new() -> Result<Self> {
        let storage = Storage::new()?;
        let todo_list = storage.load()?;
        let keybindings = KeyBindings::load()?;

        // Terminal initialization
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        
        Ok(Self {
            running: true,
            todo_list,
            storage,
            keybindings,
            selected_index: 0,
            terminal,
        })
    }

    pub fn run(&mut self) -> Result<()> {
        // Main event loop
        while self.running {
            self.terminal.draw(|f| {
                ui::draw(f, &self.todo_list, self.selected_index);
            })?;

            if let Event::Key(key) = event::read()? {
                if let Some(action) = self.keybindings.get_action(&key) {
                    match action {
                        Action::Quit => self.running = false,
                        Action::NewTodo => {
                            self.todo_list.add_todo("New Todo".to_string());
                            self.storage.save(&self.todo_list)?;
                        }
                        Action::ToggleTodo => {
                            if let Some(todo) = self.todo_list.todos.get_mut(self.selected_index) {
                                todo.completed = !todo.completed;
                                self.storage.save(&self.todo_list)?;
                            }
                        }
                        Action::DeleteTodo => {
                            if self.selected_index < self.todo_list.todos.len() {
                                self.todo_list.todos.remove(self.selected_index);
                                if self.selected_index > 0 && self.selected_index >= self.todo_list.todos.len() {
                                    self.selected_index -= 1;
                                }
                                self.storage.save(&self.todo_list)?;
                            }
                        }
                        Action::MoveTodoUp => {
                            if self.selected_index > 0 {
                                self.todo_list.todos.swap(self.selected_index, self.selected_index - 1);
                                self.selected_index -= 1;
                                self.storage.save(&self.todo_list)?;
                            }
                        }
                        Action::MoveTodoDown => {
                            if self.selected_index < self.todo_list.todos.len() - 1 {
                                self.todo_list.todos.swap(self.selected_index, self.selected_index + 1);
                                self.selected_index += 1;
                                self.storage.save(&self.todo_list)?;
                            }
                        }
                    }
                }
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        self.terminal.show_cursor()?;

        Ok(())
    }
}
