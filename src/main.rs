use rustyline::error::ReadlineError;
use rustyline::Editor; 
use std::fs;
// use std::fs::File;
// use std::env;
// use crate::fs::File;
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
                break
            }
        }
    }
    rl.save_history("history.txt").unwrap();
}

pub fn read() { 
    println!("In file {}", "history.txt"); 

    let conts = fs::read_to_string("history.txt")
        .expect("Could not read the file");
    
    println!("With text:\n{conts}"); 
}