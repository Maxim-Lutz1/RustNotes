use std::fs;
use std::io::{self, Write};

fn main() -> std::io::Result<()> {
    println!("RustNotes \nPress h for help");

    if let Some(mut path) = dirs::home_dir() {
        path.push(".rustnotes");
        path.push("notes");

        // creates (~/.rustnotes/notes) on linux
        // creates (%USERPROFILE%\.rustnotes\notes) on windows
        fs::create_dir_all(&path)?;
    } else {
        println!("Could not find the home directory.");
    }

    loop {
        let mut input = String::new();
        print!(">> ");
        io::stdout().flush().expect("Failed to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input == "exit" {
            break;
        }
        if trimmed_input == "h" {
            println!(
                "Available commands:\nh - List commands\nn - New note\nd - Delete note\nexit - Quits the program"
            )
        }
    }

    Ok(())
}
