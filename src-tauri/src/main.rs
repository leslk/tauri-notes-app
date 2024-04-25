// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Serialize, Deserialize};
use tauri::Manager;
use tauri::{Result};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};


use std::io::{Write};


#[derive(Serialize, Deserialize)]
struct Note {
    id: u32,
    title: String,
    content: String,
}

impl Note {
    fn new(id: u32, title: String, content: String) -> Self {
        Note { id, title, content }
    }
}


#[tauri::command]
fn save_note(id: usize, title: &str, content: &str) -> Result<()> {
    // Read file content and deserialize it
    let data = fs::read_to_string("notes.json").expect("Unable to read file");
    let mut notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");
    
    // Create new note to add and push it to the notes vector
    let note = Note::new(id as u32, title.to_string(), content.to_string());
    notes.push(note);

    // Serialize notes to Json data and rewrite the file with it
    let mut file = std::fs::OpenOptions::new().write(true).open("notes.json")?;
    writeln!(file, "{}", serde_json::to_string(&notes)?)?;

    Ok(())
}

#[tauri::command]
fn load_notes() -> Result<Vec<Note>> {
    // Open the file
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("notes.json")?;

    // Read the file content
    let mut data = fs::read_to_string("notes.json").expect("Unable to read file");

    // If the file is empty, write an empty array to it
    if data == "" {
        writeln!(file, "[]")?;
        data = fs::read_to_string("notes.json").expect("Unable to read file");
    }

    // Deserialize the Json data to Vec<Note>
    let notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");

    Ok(notes)
}

#[tauri::command]
fn update_note(id: usize, title: &str, content: &str) -> Result<()> {
    // open and read the file and deserialize data to Vec<Note>
    let data = fs::read_to_string("notes.json").expect("Unable to read file");
    let mut notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");

    // Find the note with the id
    let note = notes.iter_mut().find(|note| note.id == id as u32).expect("Note not found");
    
    // Update the note with the new title and content
    note.title = title.to_string();
    note.content = content.to_string();
    println!("{}, {}, {}", note.id, note.title, note.content);

    // Serialize Vec<Note> to Json
    let json = serde_json::to_string(&notes).expect("Unable to serialize");

    // Write the Json back to the file
    fs::write("notes.json", json).expect("Unable to write file");

    Ok(())
}

#[tauri::command]
fn delete_note(id: usize) -> Result<()> {
    // open and read the file and deserialize data to Vec<Note>
    let data = fs::read_to_string("notes.json").expect("Unable to read file");
    let mut notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");

    // Find the index of the note with the id
    let index = notes.iter_mut().position(|note| note.id == id as u32).expect("Note not found");
    notes.remove(index);

    // Serialize Vec<Note> to Json
    let json = serde_json::to_string(&notes).expect("Unable to serialize");

    // Write the Json back to the file
    fs::write("notes.json", json).expect("Unable to write file");

    Ok(())
}

// Créer une note
// #[tauri::command]
// fn create_note(conn: &Connection, note: &Note) ->
// Result<()> {
//     conn.execute(
//         "INSERT INTO notes (title, content) VALUES (?1, ?2)",
//         params![note.title, note.content],
// )?;
// Ok(()) }
// // Lire toutes les notes
// #[command]
// fn read_notes(conn: &Connection) -> Result<Vec<Note>> {
//     let mut stmt = conn.prepare("SELECT id, title, content FROM notes")?;
//     let note_iter = stmt.query_map([], |row| {
//         Ok((
//             row.get(0)?,
//             row.get(1)?,
//             row.get(2)?,
//         ))
//     })?;
//     let notes: Vec<Note> = note_iter.collect();
//     notes
// }
// // Mettre à jour une note
// #[command]
// fn update_note(conn: &Connection, note: &Note) ->
// Result<()> {
//     conn.execute(
//         "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
//         params![note.title, note.content, note.id],
// )?;
// Ok(()) }

// // Supprimer une note
// #[command]
// fn delete_note(conn: &Connection, id: i64) -> Result<()> {
//     conn.execute(
//         "DELETE FROM notes WHERE id = ?1",
//         params![id],
//     )?;
// Ok(()) }

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // init_db().expect("failed to initialize database");
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
                window.open_devtools();
                window.close_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            save_note,
            load_notes,
            update_note,
            delete_note
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}