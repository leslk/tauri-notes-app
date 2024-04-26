// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command



// --------------------------------- Imports ---------------------------------



use serde::{Serialize, Deserialize};
use tauri::Result;
use std::fs;
use std::io::Write;
use rusqlite::{Connection, params};
use fix_path_env;



// --------------------------------- JSON File ---------------------------------



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


// Save a note to a file
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


// Load notes from a file
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

// Update a note in the file
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

// Delete a note from the file
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



// --------------------------------- SQLite Database ---------------------------------



// create sqlite connection
fn init_db() -> Result<()> {
    // create a connection to the sqlite database
    let conn = Connection::open("notes.db").expect("failed to open database");
    // create a table in the database
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL
    )",
    [], ).expect("failed to create table");
    Ok(()) 
}

// create a note and save into sqlite DB
#[tauri::command]
fn db_save_note(title: &str, content: &str) -> Result<()> {
    // create a connection to the sqlite database
    let conn = Connection::open("notes.db").expect("failed to open database");
    // insert a note into the database
    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![title, content],
    ).expect("failed to insert note");
    Ok(())
}

// load notes from sqlite DB
#[tauri::command]
fn db_load_notes() -> Result<Vec<Note>> {
    // create a connection to the sqlite database
    let conn = Connection::open("notes.db").expect("failed to open database");
    // query all notes from the database
    let mut stmt = conn.prepare("SELECT id, title, content FROM notes").expect("failed to prepare query");
    let note_iter = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0).expect("failed to get id"),
            title: row.get(1).expect("failed to get title"),
            content: row.get(2).expect("failed to get content"),
        })
    }).expect("failed to query map");
    let notes: Vec<Note> = note_iter.map(|note| note.unwrap()).collect();
    Ok(notes)
}

// update a note in sqlite DB
#[tauri::command]
fn db_update_note(id: usize, title: &str, content: &str) -> Result<()> {
    // create a connection to the sqlite database
    let conn = Connection::open("notes.db").expect("failed to open database");
    // update a note in the database
    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
        params![title, content, id],
    ).expect("failed to update note");
    Ok(())
}

// delete a note from sqlite DB
#[tauri::command]
fn db_delete_note(id: usize) -> Result<()> {
    // create a connection to the sqlite database
    let conn = Connection::open("notes.db").expect("failed to open database");
    // delete a note from the database
    conn.execute(
        "DELETE FROM notes WHERE id = ?1",
        params![id],
    ).expect("failed to delete note");
    Ok(())
}

#[tauri::command]
fn export_notes_to_pdf() -> Result<()> {
    // create a connection to the sqlite database
    let conn = Connection::open("notes.db").expect("failed to open database");
    // query all notes from the database
    let mut stmt = conn.prepare("SELECT id, title, content FROM notes").expect("failed to prepare query");
    let note_iter = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0).expect("failed to get id"),
            title: row.get(1).expect("failed to get title"),
            content: row.get(2).expect("failed to get content"),
        })
    }).expect("failed to query map");
    let notes: Vec<Note> = note_iter.map(|note| note.unwrap()).collect();

    // create a pdf file
    let mut file = std::fs::OpenOptions::new().write(true).create(true).open("notes.pdf")?;
    writeln!(file, "Notes\n\n")?;
    for note in notes {
        writeln!(file, "Title: {}\nContent: {}\n\n", note.title, note.content)?;
    }

    Ok(())
}

fn main() {
    // fix the path environment
    let _ = fix_path_env::fix();
    // initialize the database
    init_db().expect("failed to initialize database");

    // run the tauri application
    tauri::Builder::default()
        // Add the commands to the tauri application
        .invoke_handler(tauri::generate_handler![
            save_note,
            load_notes,
            update_note,
            delete_note,
            db_save_note,
            db_load_notes,
            db_update_note,
            db_delete_note,
            export_notes_to_pdf
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}