extern crate rustyline;

use std::env;
use std::process::Command;

use rustyline::Editor;
use rustyline::error::ReadlineError;

fn main() {

    // set reader to read an input line
    let mut reader = Editor::<()>::new();

    // get user name from environment variables
    let user = env::var("USER").unwrap();

    // main loop
    loop {

        // get current working directory
        let cwd = env::var("PWD").unwrap();

        // print prompt
        print!("({}@{})\n(rushell)$ ", user, cwd);

        let prompt = format!("{}@{}\n(rushell)$ ", user, cwd);

        // read a line
        let cmd = reader.readline(prompt.as_str());

        // parse it
        match cmd {
            Ok(line) => {
                if line.trim() == "exit" {
                    println!("Bye!");
                    break;
                } else if line.trim() == "" {
                    continue;
                }

                // add the input line as a history
                reader.add_history_entry(&line);

                // split the line into Vec<&str>
                let args: Vec<&str> = line.trim().split(' ').collect();

                // execute it
                match Command::new(args[0])
                    .args(&(args[1..]))
                    .output() {
                    Ok(output) => {
                        let err = String::from_utf8_lossy(&output.stderr);
                        if err != "" {
                            print!("{}", err);
                        }
                        let out = String::from_utf8_lossy(&output.stdout);
                        if out != "" {
                            print!("{}", out);
                        }
                    },
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            },

            // caught SIGINT(Ctrl-C)
            Err(ReadlineError::Interrupted) => {
                continue;
            },

            // caught EOF(Ctrl-D)
            Err(ReadlineError::Eof) => {
                continue;
            }

            Err(err) => {
                println!("Reader Error: {:?}", err);
                continue;
            }
        }
    }
}
