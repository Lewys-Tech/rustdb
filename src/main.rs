use std::io::{self, BufRead, Write};
use std::collections::HashMap;

fn append(entry: &str){
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("db.log")
        .unwrap();
    writeln!(file, "{}", entry).unwrap();
 }

fn load() -> HashMap<String, String> {
    let mut store = HashMap::new();
    if let Ok(content) =  std::fs::read_to_string("db.log") {
        for line in content.lines() {
            let mut parts = line.splitn(2, ' ');
            let op =parts.next().unwrap_or("");
            let rest = parts.next().unwrap_or("");
            match op {
                "SET" => {
                    let mut p = rest.splitn(2, ' ');
                    let key = p.next().unwrap_or("").to_string();
                    let value = p.next().unwrap_or("").to_string();
                    store.insert(key, value);
                    append(&format!("SET {} {}", key, value));
                }
                "DELETE" => {store.remove(rest); } 
                append(&format!("SET {} {}", key, value));
                _=> {}           
            }
            
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

            "EXIT" => std::process::exit(0),
        }
    }
}    