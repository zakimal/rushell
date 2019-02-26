use std::env;
use std::io;
use std::io::Write;
use std::process::Command;

fn main() {
    // get user name from environment variables
    let user = env::var("USER").unwrap();

    // main loop
    loop {

        // get current working directory
        let cwd = env::var("PWD").unwrap();

        // print prompt
        print!("({}@{})\n(rushell)$ ", user, cwd);

        // flush it to stdout
        io::stdout().flush().unwrap();

        // read commandline input
        let mut cmd = String::new();
        io::stdin().read_line(&mut cmd)
            .expect("Error: failed to read line");

        if cmd.trim() == "exit" {
            println!("Bye!");
            break;
        } else if cmd.trim() == "" {
            continue;
        }

        // parse the arguments
        let args: Vec<&str> = cmd.trim().split(' ').collect();

        // execute them
        let output = Command::new(args[0])
            .args(&(args[1..]))
            .output()
            .expect("Error: failed to run");

        // print the output
        print!("{}", String::from_utf8(output.stdout).unwrap());
    }
}
