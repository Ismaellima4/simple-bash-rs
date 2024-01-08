use std::{
    env::{self},
    fs,
    io::{self, stdout, Write},
    path::Path,
};

fn main() -> Result<(), String> {
    loop {
        let current_dir = match env::current_dir() {
            Err(e) => return Err(format!("Error: {}", e)),
            Ok(path) => path,
        };

        print!("{}$ ", current_dir.to_string_lossy());

        match io::stdout().flush() {
            Ok(_) => (),
            Err(e) => return Err(format!("Error: {}", e)),
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                if n <= 1 {
                    return Ok(());
                }
            }
            Err(e) => return Err(format!("Error: {}", e)),
        }

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&root) {
                    eprintln!("Error: {}", e);
                }
            }
            "exit" => return Ok(()),
            "ls" => {
                let paths = fs::read_dir(&current_dir).unwrap();
                let _ = paths
                    .map(|path| {
                        print!("{} ", path.unwrap().file_name().into_string().unwrap());
                    })
                    .collect::<Vec<_>>();
                println!("");
            }
            "clear" | "cls" => {
                println!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));
                stdout().flush().unwrap();
            }
            _ => println!("command {} not found:", &command),
        }
    }
}
