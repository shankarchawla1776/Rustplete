use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs;
use std::io::Write;
use chrono::prelude::*;

fn main() {
    let mut rl = Editor::<()>::new();
    let _history_file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Unable to open or create history.txt");
    println!("history.txt created in the current directory.");

    // file.write_all(b"")
    //     .expect("Unable to write to file");

    if rl.load_history("history.txt").is_err() {
        println!("No previous history");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(ref line) => {
                rl.add_history_entry(line.as_str());
                println!("Line: {}", line);
                println!("Do you wish to add this note to your notebook? (y/n)");
                let mut ans = String::new();
                if let Err(_) = std::io::stdin().read_line(&mut ans) {
                    println!("Error reading input");
                    break;
                }
                let ans = ans.trim();
                match ans {
                    "y" => {
                        if let Err(e) = to_notes(line, "notes.txt") {
                            println!("Error {:?}", e)
                        }
                    }
                    "n" => {
                        println!("Note not saved to notebook");
                    }
                    _ => {
                        println!("Invalid Input Error: please answer yes (y) or no (n): {:?}", ans)
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

pub fn to_notes(line: &str, notes_file: &str) -> std::io::Result<()> {
    let local: DateTime<Local> = Local::now();
    let local_time = local.format("%Y-%m-%d %H:%M:%S").to_string();
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(notes_file)?;
    writeln!(file, "{}: {}", local_time, line)?;
    println!("Note saved to {}", notes_file);
    println!("Press CRTL-C to exit.");
    Ok(())
}
