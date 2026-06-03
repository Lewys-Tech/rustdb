use std::io::{self, BufRead, Write};
use std::collections::HashMap;

fn save(store: &HashMap<String, String>){
    let mut file = std::fs::File::create("db.txt").unwrap();
    for (key, value) in store.iter() {
        writeln!(file, "{} {}", key, value).unwrap();
    }
}

fn load() -> HashMap<String, String> {
    let mut store = HashMap::new();
    if let Ok(content) =  std::fs::read_to_string("db.txt") {
        for line in content.lines() {
            let mut parts = line.splitn(2, ' ');
            let key = parts.next().unwrap_or("").to_string();
            let value = parts.next().unwrap_or("").to_string();
            store.insert(key, value);
        }
    }
    store
}

fn main() {
    let mut store = load();
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

            "EXIT" => { save(&store); std::process::exit(0);}
            _ => println!("Unknown command: {}", cmd),
        }
    }
}    