use serde::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::env;
use std::process::Command;
use std::str::FromStr;
use tempfile::NamedTempFile;
use directories::ProjectDirs;
use std::path::PathBuf;
use ansi_term::Color::{Black, Green, Yellow};

#[derive(Serialize, Deserialize, Debug)]
struct Item {
    item: String,
    note: String,
    completed: bool,
}

static DATA_FILE: &'static str = "data.json";

/**
 * ToDo item structure:
 *      "item": string
 *      "note": string,
 *      "completed": bool
 *
 *  Example:
 *  [
 *      {"item": "First task", "note": "This is a more in-depth description", "completed": false},
 *      {"item": "Second task", "note": "This is a more in-depth description", "completed": true},
 *      {"item": "Third task", "note": "This is a more in-depth description", "completed": true},
 *      {"item": "Fourth task", "note": "This is a more in-depth description", "completed": false}
 *  ]
 *
 */

fn main()  {
    if let Some(data_dir) = get_data_dir() {
        let fp = data_dir.join(DATA_FILE);
        let file_path = fp.to_str().unwrap();

        let mut file = File::open(file_path).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();

        if content.is_empty() {
            fs::write(file_path, "[]").unwrap();
            content = String::from_str("[]").unwrap();
        }

        let mut json: Vec<Item> = serde_json::from_str(content.as_str()).unwrap();

        if let Some(arg) = std::env::args().nth(1) {
            match arg.as_str() {
               "--list" | "-l" => list_tasks(json),
               "--help" | "-h" => print_help(),
               "--preview" | "-p" => preview_task(json),
               "--add" | "-a" => add_task(&mut json, file_path),
               "--edit-note" | "-en" => edit_note(&mut json, file_path),
               "--edit-title" | "-et" => edit_title(&mut json, file_path),
               "--toggle" | "-t" => toggle_item(&mut json, file_path),
               "--delete" | "-d" => delete_item(&mut json, file_path),
               "--erase-all" => erase_all(file_path),
               _ => print_help(),
            };
        } else {
            print_help();
        }
    } else {
        eprintln!("Could not determine data directory");
    }

    
}

fn erase_all(file: &str)  {
    print!("Are you sure you want to erase all items? [yN] ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let response = buf.trim();

    if response.len() > 0 && response.to_lowercase().chars().nth(0).unwrap() == 'y' {
        fs::write(file, "[]").unwrap();
    }

    
}

fn get_data_dir() -> Option<PathBuf> {
    if let Some(proj_dirs) = ProjectDirs::from("com", "GhoulKingR", "cli-todo") {
        let data_dir = proj_dirs.data_dir();

        // Create the directory if it doesn't exist
        if !data_dir.exists() {
            fs::create_dir_all(data_dir).expect("Failed to create data directory");
        }

        Some(data_dir.to_path_buf())
    } else {
        None
    }
}

fn delete_item(json: &mut Vec<Item>, file: &str)  {
    let index = get_second_arg(&json).expect("Failed to get second argument");
    let item = &json[index];
    print!("Are you sure you want to delete this item \"{}\"? [yN] ", item.item);
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let response = buf.trim();

    if response.len() > 0 && response.to_lowercase().chars().nth(0).unwrap() == 'y' {
        json.remove(index);
        save(&json, file);
    }

    
}

fn toggle_item(json: &mut Vec<Item>, file: &str)  {
    let index = get_second_arg(&json).expect("Failed to get second argument");
    let item = &mut json[index];
    println!("Title: {}\nOriginal: {:?}\nCurrent: {:?}", item.item, item.completed, !item.completed);
    item.completed = !item.completed;
    save(&json, file);

    
}

fn get_second_arg(json: &Vec<Item>) -> Result<usize, Box<dyn std::error::Error>> {
    let index_arg: usize = std::env::args().nth(2).ok_or("Missing argument").unwrap().parse().unwrap();
    let index = index_arg - 1;

    if index >= json.len() {
        Err("Index out of range".into())
    } else {
        Ok(index)
    }
}

fn save(json: &Vec<Item>, file: &str)  {
    let string = serde_json::to_string(&json).unwrap();
    fs::write(file, string.as_str()).unwrap();
    
}

fn edit_title(json: &mut Vec<Item>, file: &str)  {
    let index = get_second_arg(&json).expect("Failed to get second argument");
    let item = &mut json[index];
        
    println!("Original title: {}", item.item);
    print!("New title (Leave empty to leave unchanged): ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    let new_item = buf.trim().to_string();
    if new_item.len() > 0 {
        item.item = new_item;
        save(&json, file);
    }
    
}

fn edit_note(json: &mut Vec<Item>, file: &str)  {
    let index = get_second_arg(&json).expect("Failed to get second argument");
    let item = &mut json[index];
    item.note = get_note(item.note.as_str());
    save(&json, file);
    
}

fn add_task(json: &mut Vec<Item>, file: &str)  {
    print!("Enter item title: ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");

    let note = get_note("# Add a note to the todo item (Remember to remove this line before saving)");
    let new_item = Item {
        item: buf.trim().to_string(),
        note: note.trim().to_string(),
        completed: false,
    };

    json.push(new_item);
    save(&json, file);

}

fn get_note(content: &str) -> String {
    let temp_file = NamedTempFile::new().expect("Cannot create temp file");
    let temp_path = temp_file.path().to_owned();

    fs::write(&temp_path, content.as_bytes())
        .expect("Cannot write to temp file");

    let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());

    Command::new(editor)
        .arg(&temp_path)
        .status()
        .expect("Failed to open the editor");

    let edited_content = fs::read_to_string(temp_path).expect("Cannot read temp file");

    return edited_content;
}

fn preview_task(json: Vec<Item>) {
    let index = get_second_arg(&json).expect("Failed to get second argument");
    let item = &json[index];

    println!("Title: {}", item.item);
    println!("Completed: {:?}\n", item.completed);
    println!("{}", item.note);
}

fn print_help() {
   println!(
       "A CLI ToDo list app

Usage: {} [OPTION]

Options:
    --help, -h                      Display this help menu
    --list, -l                      List all items in the todo list
    --add, -a                       Interactively add a new todo item
    --preview ITEM, -p ITEM         Preview an item in more detail
    --edit-note ITEM, -en ITEM      Edit an item note
    --edit-title ITEM, -et ITEM     Edit an item's title
    --toggle ITEM, -t ITEM          Toggle complete status of an item
    --delete ITEM, -d ITEM          Delete an item from the todo list
    --erase-all                     Delete every item in the todo list
",
        std::env::args().nth(0).unwrap()
    );
    
}

fn list_tasks(json: Vec<Item>) {
    println!("All items:");
    for (index, item) in json.iter().enumerate() {
        let completed_text = if item.completed {
            Black.on(Green).paint("[x]")
        } else {
            Black.on(Yellow).paint("[ ]")
        };
        println!("{}. {} {}", index + 1, completed_text, item.item);
    }
}
