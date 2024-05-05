# TAURI_NOTES_APP 📝
This is a simple notes application built with Rust, utilizing several libraries and tools for various functionalities.

## Features
Create, read, update, and delete (CRUD) notes.
Cross-platform desktop application using Tauri.

## Technologies Used
- Rust: The programming language used for the backend.
- Tauri: A framework for building cross-platform desktop applications with web technologies.
- Rusqlite: A SQLite library for Rust.
- Serde: A serialization/deserialization library for Rust.
- Directories: A Rust library to easily access common platform-specific directories.
- Fix-path-env: A library to fix issues related to paths on different operating systems (especially on macOs).

## Getting Started
1. Clone the repository:
    - git clone https://github.com/your-username/notes-app.git
2. Install Rust and Cargo if you haven't already. Refer to the official Rust website for installation instructions.
    - [https://www.rust-lang.org/tools/install](https://doc.rust-lang.org/cargo/getting-started/installation.html)
3. Install Tauri CLI:
    - cargo install tauri-cli
4. Install project dependencies en effectuant les commandes suivantes vie le terminal:
   - cd src-tauri
   - cargo build
5. Run the application in development:
    - cargo tauri dev
6. Build the application:
    - cargo tauri build

## Usage
Open the application.
Create, view, update, or delete notes as needed.


