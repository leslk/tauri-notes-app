// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command



// --------------------------------- Imports ---------------------------------



use serde::{Serialize, Deserialize};
use serde_json::to_string;
use tauri::{ Result};
use std::fs;
use std::io::Write;
use rusqlite::{Connection, params, Result as RusResult};
use fix_path_env;
use std::path::PathBuf;
use directories::UserDirs;
use chrono::{Local, DateTime};




// --------------------------------- JSON File ---------------------------------



#[derive(Serialize, Deserialize)]

/// A struct to represent a note
struct Note {
    /// The id of the note
    id: u32,
    /// The title of the note
    title: String,
    // The content of the note
    content: String,
}


/// Implement the Note struct
impl Note {
    /// Create a new note
    fn new(id: u32, title: String, content: String) -> Self {
        Note { id, title, content }
    }
}


// Save a note to a file

/// This function saves a note to a file
/// # Arguments
/// * `id` - The id of the note
/// * `title` - The title of the note
/// * `content` - The content of the note
/// * `app_handle` - The handle to the tauri application
#[tauri::command]
fn save_note(id: usize, title: &str, content: &str, app_handle: tauri::AppHandle) -> Result<()> {
    // Read file content and deserialize it
    let respath = get_json_path(app_handle);
    let data = fs::read_to_string(&respath).expect("Unable to read file");
    let mut notes: Vec<Note> = Vec::new();
    if data.is_empty() {
        fs::write(&respath, "[]").expect("Unable to write file");
    } else {
        notes = serde_json::from_str(&data).expect("Unable to deserialize");
    }
    
    // Create new note to add and push it to the notes vector
    let note = Note::new(id as u32, title.to_string(), content.to_string());
    notes.push(note);

    // Serialize notes to Json data and rewrite the file with it
    let mut file = std::fs::OpenOptions::new().write(true).open(&respath)?;
    writeln!(file, "{}", serde_json::to_string(&notes)?)?;

    Ok(())
}


// Load notes from a file

/// This function loads notes from a file
/// # Arguments
/// * `app_handle` - The handle to the tauri application
/// # Returns
/// * A Result containing the notes
#[tauri::command]
fn load_notes(query_search: &str, app_handle: tauri::AppHandle) -> Result<Vec<Note>> {
    // Open the file
    let respath = get_json_path(app_handle);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(&respath)?;

    // Read the file content
    let mut data = fs::read_to_string(&respath).expect("Unable to read file");

    // If the file is empty, write an empty array to it
    if data == "" {
        writeln!(file, "[]")?;
        data = fs::read_to_string(&respath).expect("Unable to read file");
    }

    // Deserialize the Json data to Vec<Note>
    let notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");
    // Filter notes based on the query search to lower case
    let notes: Vec<Note> = notes.into_iter().filter(|note| note.title.to_lowercase().contains(&query_search.to_lowercase()) || note.content.to_lowercase().contains(&query_search.to_lowercase())).collect();
    Ok(notes)
}

// Update a note in the file

/// This function updates a note in the file
/// # Arguments
/// * `id` - The id of the note
/// * `title` - The title of the note
/// * `content` - The content of the note
/// * `app_handle` - The handle to the tauri application
#[tauri::command]
fn update_note(id: usize, title: &str, content: &str, app_handle: tauri::AppHandle) -> Result<()> {
    // open and read the file and deserialize data to Vec<Note>
    let respath = get_json_path(app_handle);
    let data = fs::read_to_string(&respath).expect("Unable to read file");
    let mut notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");

    // Find the note with the id
    let note = notes.iter_mut().find(|note| note.id == id as u32).expect("Note not found");
    
    // Update the note with the new title and content
    note.title = title.to_string();
    note.content = content.to_string();

    // Serialize Vec<Note> to Json
    let json = serde_json::to_string(&notes).expect("Unable to serialize");

    // Write the Json back to the file
    fs::write(&respath, json).expect("Unable to write file");

    Ok(())
}

// Delete a note from the file

/// This function deletes a note from the file
/// # Arguments
/// * `id` - The id of the note
/// * `app_handle` - The handle to the tauri application
#[tauri::command]
fn delete_note(id: usize, app_handle: tauri::AppHandle) -> Result<()> {
    // open and read the file and deserialize data to Vec<Note>
    let respath = get_json_path(app_handle);
    let data = fs::read_to_string(&respath).expect("Unable to read file");
    let mut notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");

    // Find the index of the note with the id
    let index = notes.iter_mut().position(|note| note.id == id as u32).expect("Note not found");
    notes.remove(index);

    // Serialize Vec<Note> to Json
    let json = serde_json::to_string(&notes).expect("Unable to serialize");

    // Write the Json back to the file
    fs::write(&respath, json).expect("Unable to write file");

    Ok(())
}

#[tauri::command]
fn export_notes_to_json(app_handle: tauri::AppHandle) -> Result<()> {
    // open and read the file and deserialize data to Vec<Note>
    let respath = get_json_path(app_handle);
    let data = fs::read_to_string(&respath).expect("Unable to read file");
    let notes: Vec<Note> = serde_json::from_str(&data).expect("Unable to deserialize");

    let user_dirs: UserDirs = UserDirs::new().unwrap();
    let downloads_dir: String = user_dirs.download_dir().unwrap().to_str().unwrap().to_string();
    println!("Downloads Dir: {}", downloads_dir);
    let mut json_path = PathBuf::new();
    // create timestamp 
    let date: DateTime<Local> = Local::now();
    let timestamp = date.timestamp().to_string();
    json_path.push(&downloads_dir);
    json_path.push(timestamp.to_string() + "_notes.json");

    let mut file = std::fs::OpenOptions::new().write(true).create(true).open(&json_path)?;
    let notes_json = serde_json::to_string(&notes).expect("Unable to serialize");
    file.write_all(notes_json.as_bytes())?;

    let _ = std::process::Command::new("open")
        .arg("-R") // Opens the Finder at the location of the file
        .arg(&json_path)
        .output();
    
    #[cfg(not(target_os = "macos"))]
    let _ = std::process::Command::new("explorer")
        .arg(&json_path)
        .output();

    Ok(())
}



// --------------------------------- SQLite Database ---------------------------------



// create sqlite connection

/// This function initializes the sqlite database
/// # Arguments
/// * `path` - The path to the sqlite database
fn init_db(path: String) -> RusResult<()> {
    // create a connection to the sqlite database
    let conn = Connection::open(&path).expect("DB Connection Err");
    // create a table in the database
    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            content TEXT NOT NULL
    )",
    [], ).expect("DB Create Err");
    Ok(()) 
}

// create a note and save into sqlite DB

/// This function creates a note and saves it into the sqlite database
/// # Arguments
/// * `title` - The title of the note
/// * `content` - The content of the note
/// * `app_handle` - The handle to the tauri application
#[tauri::command]
fn db_save_note(title: &str, content: &str, app_handle: tauri::AppHandle) -> Result<()> {
    // create a connection to the sqlite database
    let respath = get_db_path(app_handle);
    let conn = Connection::open(&respath).unwrap();
    println!("DB Path: {}", respath);
    println!("Title: {}", title);
    println!("Content: {}", content);
    // insert a note into the database
    conn.execute(
        "INSERT INTO notes (title, content) VALUES (?1, ?2)",
        params![title, content],
    ).unwrap_or_else(|_| panic!("failed to insert note"));
    Ok(())
}

// load notes from sqlite DB

/// This function loads notes from the sqlite database
/// # Arguments
/// * `app_handle` - The handle to the tauri application
/// # Returns
/// * A Result containing the notes
#[tauri::command]
fn db_load_notes(query_search: &str, app_handle: tauri::AppHandle) -> Result<Vec<Note>> {
    // create a connection to the sqlite database
    let respath = get_db_path(app_handle);
    let conn = Connection::open(&respath).unwrap();
    // query all notes from the database
    let mut stmt = conn.prepare("SELECT id, title, content FROM notes WHERE content LIKE '%' || ? || '%' OR title LIKE '%' || ? || '%'").unwrap_or_else(|_| panic!("failed to prepare query"));
    let note_iter = stmt.query_map(params![query_search, query_search], |row| {
        Ok(Note {
            id: row.get(0).expect("failed to get id"),
            title: row.get(1).expect("failed to get title"),
            content: row.get(2).expect("failed to get content"),
        })
    }).unwrap_or_else(|_| panic!("failed to query map"));
    let notes: Vec<Note> = note_iter.map(|note| note.unwrap()).collect();
    Ok(notes)
}

// update a note in sqlite DB

/// This function updates a note in the sqlite database
/// # Arguments
/// * `id` - The id of the note
/// * `title` - The title of the note
/// * `content` - The content of the note
/// * `app_handle` - The handle to the tauri application
#[tauri::command]
fn db_update_note(id: usize, title: &str, content: &str, app_handle: tauri::AppHandle) -> Result<()> {
    // create a connection to the sqlite database
    let respath = get_db_path(app_handle);
    let conn = Connection::open(&respath).unwrap();
    // update a note in the database
    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
        params![title, content, id],
    ).expect("failed to update note");
    Ok(())
}

// delete a note from sqlite DB

/// This function deletes a note from the sqlite database
/// # Arguments
/// * `id` - The id of the note
/// * `app_handle` - The handle to the tauri application
#[tauri::command]
fn db_delete_note(id: usize, app_handle: tauri::AppHandle) -> Result<()> {
    // create a connection to the sqlite database
    let respath = get_db_path(app_handle);
    let conn = Connection::open(&respath).unwrap();
    // delete a note from the database
    conn.execute(
        "DELETE FROM notes WHERE id = ?1",
        params![id],
    ).expect("failed to delete note");
    Ok(())
}

#[tauri::command]
/// This function exports notes to a pdf file
/// # Arguments
/// * `app_handle` - The handle to the tauri application
fn db_export_notes_to_json(app_handle: tauri::AppHandle) -> Result<()> {
    // create a connection to the sqlite database
    let respath = get_db_path(app_handle);
    let conn = Connection::open(&respath).unwrap();
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
    let user_dirs: UserDirs = UserDirs::new().unwrap();
    let downloads_dir: String = user_dirs.download_dir().unwrap().to_str().unwrap().to_string();
    println!("Downloads Dir: {}", downloads_dir);
    let mut json_path = PathBuf::new();
    // create timestamp 
    let date: DateTime<Local> = Local::now();
    let timestamp = date.timestamp().to_string();
    json_path.push(&downloads_dir);
    json_path.push(timestamp.to_string() + "_notes.json");

    let mut file = std::fs::OpenOptions::new().write(true).create(true).open(&json_path)?;
    let notes_json = serde_json::to_string(&notes).expect("Unable to serialize");
    file.write_all(notes_json.as_bytes())?;

    let _ = std::process::Command::new("open")
        .arg("-R") // Opens the Finder at the location of the file
        .arg(&json_path)
        .output();
    
    #[cfg(not(target_os = "macos"))]
    let _ = std::process::Command::new("explorer")
        .arg(&json_path)
        .output();

    Ok(())
}

/// This function returns the path to the sqlite database
/// This allow us to use the same database path in all the functions
fn get_db_path(app: tauri::AppHandle) -> String {
    return app.path_resolver()
        .app_data_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/notes.db";
}

/// This function returns the path to the json file
/// This allow us to use the same json file path in all the functions
fn get_json_path(app: tauri::AppHandle) -> String {
    return app.path_resolver()
        .app_data_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap() + "/notes.json";
}


/// The main function
/// This function initializes the database and runs the tauri application
/// It also sets up the path environment
/// It also adds the commands to the tauri application and runs it
fn main() {
    // fix the path environment
    let _ = fix_path_env::fix();
    // run the tauri application
    tauri::Builder::default()
        .setup(|app| {
            fs::create_dir_all(app.path_resolver().app_data_dir().unwrap());
            let respath = get_db_path(app.handle());
            // initialize the database
            init_db(respath);
            Ok(())
        })
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
            db_export_notes_to_json,
            export_notes_to_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}