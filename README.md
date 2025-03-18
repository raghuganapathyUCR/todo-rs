# Todo-rs 📝

A terminal-based todo application written in Rust, inspired by taskell. Features vim-like keybindings and a clean, intuitive interface.

## Features ✨

- Vim-style navigation and controls
- Customizable keybindings
- Persistent storage
- Terminal UI with visual feedback
- Task completion tracking
- Task reordering

## Keybindings 🎮

Default keybindings (can be customized via `~/.config/todo-rs/keybindings.json`):

| Key    | Action         |
|--------|---------------|
| `j`    | Move down     |
| `k`    | Move up       |
| `space`| Toggle task   |
| `d`    | Delete task   |
| `n`    | New task      |
| `q`    | Quit          |

## Installation 🚀

Ensure you have Rust installed ([rustup](https://rustup.rs/)), then:

```bash
# Clone the repository
git clone https://github.com/raghuganapathyUCR/todo-rs.git
cd todo-rs

# Build and install
cargo install --path .
```

## Configuration 🛠️

The application creates a configuration directory at `~/.config/todo-rs/` with:

- `keybindings.json`: Customize your key mappings
- `todos.json`: Your tasks are stored here

## Development 🔧

Built with:
- [ratatui](https://crates.io/crates/ratatui) - Terminal UI framework
- [crossterm](https://crates.io/crates/crossterm) - Terminal manipulation

## Contributing 
Contributions are welcome! Feel free to:
1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License 

MIT License - feel free to use this project however you'd like!
