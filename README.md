<a> ![readme banner](</assets/banner.png>) </a>

<div align=center>
 
![Rust](https://img.shields.io/badge/Built%20with-Rust-800000?style=flat-square&logo=rust&logoColor=white)
![macOS](https://img.shields.io/badge/Target-macOS-8B0000?style=flat-square&logo=apple&logoColor=white)
![Open Source](https://img.shields.io/badge/Open%20Source-Community-660000?style=flat-square&logo=github&logoColor=white)
</div>

# Astra Shell

A modern shell built from the ground up for macOS.

Astra is an interactive command-line environment focused on a clean interface, customization, and a better terminal experience. It combines the power of traditional Unix shells with a modern prompt system, configuration, and extensibility.

>[!WARNING]
> Astra Shell has not gone through extensive testing yet. <br>
> Wait until the first working release to start using, use at your own risk.
## Features

- Custom interactive shell
- Fast Rust-based core
- Powerlevel10k-inspired prompt system
- Git-aware prompt information
- Command history
- Tab completion
- Custom configuration
- Theme support
- Built-in commands
- Modular architecture

## Screenshots

Coming soon.

## Installation

### Requirements

- macOS
- Rust toolchain

Install Rust if you do not already have it:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
Clone Astra:
```bash
git clone https://github.com/yourusername/astra.git
cd astra
```
Build:
```bash
cargo build --release
```
Run:
```bash
./target/release/astra
```
### Usage
Start Astra:
```bash
astra
```
Example:
```
╭─ Astra
  git: main
 ◉ user@MacBook-Air
 ~/Developer/Astra
╰─❯
```
Astra can execute normal macOS commands:
```bash
ls
cd Projects
git status
python3 script.py
```
Built-in commands:
```bash
cd
pwd
clear
exit
Configuration
```
Astra uses a configuration file located at:
```bash
~/.astrarc
```
Example:
```toml
theme = "default"
git_prompt = true
history_size = 5000
```
Configuration controls how Astra behaves and allows users to customize their terminal experience.

### Themes

Astra supports customizable themes.

Themes are stored in `themes/`

Example:
```
themes/
   - default.toml
   - minimal.toml (coming soon)
   - cyberpunk.toml (coming soon)
```
A theme can control:

- Prompt layout
- Icons
- Colors
- Display information
- Architecture

Astra is designed to be modular.
```
src/
   - main.rs
   - shell.rs
   - prompt.rs
   - parser.rs
   - executor.rs
   - builtins.rs
   - history.rs
   - config.rs
   - theme.rs
   - git.rs
   - completion.rs
```
Each component handles a specific part of the shell, making Astra easier to expand and maintain.

## Why Astra?

Existing shells are powerful, but many rely on years of configuration and plugins to create a modern experience.

Astra aims to provide:

- A clean default experience
- Strong customization
- Modern terminal features
- A foundation for future extensions

The goal is not to replace every shell. The goal is to create a shell that feels modern from the first launch.

## Contributing

**Contributions are welcome.**

If you want to help improve Astra:

- Fork the repository
- Create a branch
- git checkout -b feature-name
- Make your changes
- Commit your work
- git commit -m "Add feature"
- Open a pull request
  
## License

Astra is licensed under the MIT License.

See [LICENSE](https://github.com/SYOP200/astra-shell/blob/main/LICENSE) for more information.

## Status

Astra is currently in early development.

The project is focused on building a stable shell foundation while improving the interactive terminal experience.
