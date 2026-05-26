# topo

A "htop" clone built in Rust to learn TUI development and system interaction.

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![Rust](https://img.shields.io/badge/language-Rust-orange.svg)

<img width="1132" height="145" alt="image" src="https://github.com/user-attachments/assets/6b22cd74-433a-4bbc-bec2-2d48c2b2ebfc" />

## Features

- **Real-time Monitoring**: Tracks memory usage and active processes.
- **Process List**: Displays PID, Name, CPU usage, and Memory consumption.
- **Interactive TUI**: Navigate through processes and monitor system health in a terminal-based interface.
- **Cross-platform**: Powered by `sysinfo` and `ratatui`.

## Tech Stack

- [Ratatui](https://ratatui.rs/) - TUI framework.
- [sysinfo](https://github.com/GuillaumeGomez/sysinfo) - System information collection.
- [Crossterm](https://github.com/crossterm-rs/crossterm) - Terminal backend.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended).

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/topo.git
   cd topo
   ```

2. Run the application:
   ```bash
   cargo run
   ```

## Keybindings

| Key | Action |
|-----|--------|
| `q` | Quit application |
| `↑` / `k` | Navigate up |
| `↓` / `j` | Navigate down |

## Roadmap

- I really don't know yet, but something chill

## License

MIT
