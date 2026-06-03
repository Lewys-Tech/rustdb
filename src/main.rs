use std::io::{self, BufRead, Write};
use std::collections::HashMap;



fn main() {
    let mut store: HashMap<String, String> = HashMap::new();
    let stdin = io::stdin();

    loop {
        print!("rush> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        stdin.lock().read_line(&mut line).unwrap();
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let mut parts = line.split_whitespace();
        let cmd = parts.next().unwrap_or("");
        let args: Vec<&str> = parts.collect();

        match cmd {
            "SET" => {
                if args.len() < 2 {
                    println!("Usage: SET <key> <value>");
                    continue;
                }
                let key = args[0].to_string();
                let value = args[1].to_string();
                store.insert(key, value);
                println!("OK");
            }

            "GET" => {
                if args.len() < 1 {
                    println!("Usage: GET <key> ");
                    continue;
                }
                let key = args[0].to_string();
                match store.get(&key){
                    Some(value) => println!("{}", value),
                    None        => println!("(nil)"),
                }
                
                
            }
            "DELETE" => {
                let key = args[0].to_string();
                store.remove(&key);
                println!("OK");
            }

            "EXIT" => std::process::exit(0),
            _ => println!("Unknown command: {}", cmd),
        }
    }
}    