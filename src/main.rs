use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::fs;
use std::io::Write;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut file = std::fs::File::create("history.txt")
        .expect("Unable to create .txt file");

    println!("history.txt created in the current directory.");

    file.write_all(b"")
        .expect("Unable to write to file");

    if rl.load_history("history.txt").is_err() {
        println!("No previous history");
    }
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
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
                        if let Err(e) = to_notes("history.txt", "notes.txt") {
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

pub fn to_notes(history_file_path: &str, notes_file: &str) -> std::io::Result<()> {
    let conts = fs::read_to_string(history_file_path)?;
    let mut file = fs::File::create(notes_file)?;
    file.write_all(conts.as_bytes())?;
    println!("Note saved to {}", notes_file);
    println!("Press CRTL-C to exit.");
    Ok(())
}
