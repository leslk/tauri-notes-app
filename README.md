# TAURI_NOTES_APP 📝

This is a simple notes application built with Rust, utilizing several libraries and tools for various functionalities.

## Features

Create, read, update, and delete (CRUD) notes.
Export notes to json file.
Cross-platform desktop application using Tauri.

## Technologies Used

- Rust: The programming language used for the backend.
- Tauri: A framework for building cross-platform desktop applications with web technologies.
- Rusqlite: A SQLite library for Rust.
- Serde: A serialization/deserialization library for Rust.
- Directories: A Rust library to easily access common platform-specific directories.
- Fix-path-env: A library to fix issues related to paths on different operating systems (especially on macOs).
- chrono: A Rust library for parsing, formatting, and manipulating dates and times.

## Getting Started

1. Clone the repository:

```shell
git clone git@github.com:leslk/tauri-notes-app.git
```

2. Install Rust and Cargo if you haven't already. Refer to the official Rust website for installation instructions.
   - [https://www.rust-lang.org/tools/install](https://doc.rust-lang.org/cargo/getting-started/installation.html)
3. Install Tauri CLI:

```shell
cargo install tauri-cli
```

4. Install project dependencies en effectuant les commandes suivantes vie le terminal:

```shell
cd src-tauri
cargo build
```

5. Run the application in development:

```shell
cargo tauri dev
```

6. Build the application:

```shell
cargo tauri build
```
