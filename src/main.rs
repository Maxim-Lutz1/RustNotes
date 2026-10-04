use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

fn get_notes_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|mut path| {
        path.push(".rustnotes");
        path.push("notes");
        path
    })
}

fn main() -> io::Result<()> {
    println!("RustNotes \nPress h for help");

    let notes_dir = match get_notes_dir() {
        Some(dir) => {
            fs::create_dir_all(&dir)?;
            dir
        }
        None => {
            eprintln!("Could not find the home directory.");
            return Ok(());
        }
    };

    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush()?;

        io::stdin().read_line(&mut input)?;
        let trimmed_input = input.trim();

        match trimmed_input {
            "exit" => break,
            "h" => println!(
                "Available commands:\nh - List commands\nn - New note\nd - Delete note\nexit - Quits the program"
            ),
            "n" => new_note(&notes_dir),
            _ => {}
        }
    }

    Ok(())
}

fn new_note(notes_dir: &PathBuf) {
    let mut title_input = String::new();

    print!("Title of new note: ");
    if io::stdout().flush().is_err() {
        eprintln!("Failed to flush output.");
        return;
    }

    if io::stdin().read_line(&mut title_input).is_err() {
        eprintln!("Failed to read line.");
        return;
    }

    let title = title_input.trim();
    if title.is_empty() {
        println!("Note title cannot be empty.");
        return;
    }

    let file_path = notes_dir.join(format!("{}.txt", title));

    match File::create(&file_path) {
        Ok(_) => println!("Created note: {}", file_path.display()),
        Err(e) => eprintln!("Failed to create note: {}", e),
    }
}
