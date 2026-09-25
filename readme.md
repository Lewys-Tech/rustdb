rustdb 🗄️

A persistent key-value database engine built from scratch in Rust using nothing but the standard library. No external crates, no SQLite, no serde — just raw file I/O, a HashMap, and an append-only log.

Built to understand how real databases like LevelDB, RocksDB, and Bitcask work under the hood.

Demo
rustdb> SET name lewis
OK
rustdb> SET city nairobi
OK
rustdb> GET name
lewis
rustdb> DELETE city
OK
rustdb> GET city
(nil)
rustdb> COMPACT
Compacted 1 keys
rustdb> EXIT

Restart the program — your data is still there:

rustdb> GET name
lewis
Features
In-memory store — instant reads via a HashMap
Persistence — data survives restarts via an append-only log
Append-only log — every write is appended, nothing is ever overwritten
Compaction — clean up the log, keeping only the latest value per key
REPL interface — interactive command line
Getting Started
Prerequisites
Rust (1.70 or later)
Build and run
bash
git clone https://github.com/yourusername/rustdb
cd rustdb
cargo run
Commands
Command	Example	Description
SET <key> <value>	SET name lewis	Store a key-value pair
GET <key>	GET name	Retrieve a value by key
DELETE <key>	DELETE name	Remove a key
COMPACT	COMPACT	Clean up the log file
EXIT	EXIT	Exit the database
How It Works
Architecture
                ┌─────────────────────┐
  User input    │       REPL          │
  ─────────────►│  SET / GET / DELETE │
                └────────┬────────────┘
                         │
              ┌──────────▼──────────┐
              │   In-memory Store   │
              │   HashMap<K, V>     │
              └──────────┬──────────┘
                         │
              ┌──────────▼──────────┐
              │   Append-only Log   │
              │      db.log         │
              └─────────────────────┘
In-memory HashMap

All reads (GET) go directly to the HashMap — instant lookup regardless of how large the log file is.

Append-only log

Every write (SET, DELETE) is immediately appended to db.log:

SET name lewis
SET city nairobi
SET name bob       ← overwrites lewis in memory
DELETE city        ← removes city from memory

Nothing is ever overwritten in the log — new entries always go to the end. This means:

Writes are extremely fast — just one disk append
Crashes can't corrupt existing data
You have a complete history of every change
Loading on startup

When the program starts, load() replays the log from top to bottom:

rust
match op {
    "SET"    => store.insert(key, value),
    "DELETE" => store.remove(key),
}

The last write for each key wins — so the in-memory state is always correct.

Compaction

Over time the log grows large with outdated entries. COMPACT rewrites db.log keeping only the latest value for each key:

Before compaction:

SET name lewis
SET city nairobi
SET name bob

After compaction:

SET city nairobi
SET name bob

This is exactly how Bitcask (the storage engine behind Riak) works.

File Format

db.log is a plain text, newline-delimited file. Each line is one operation:

SET <key> <value>
DELETE <key>

You can inspect it directly:

bash
cat db.log

Or even edit it manually — the database will replay whatever is in the file on next startup.

Key Rust Concepts
Concept	Where used
HashMap<String, String>	in-memory key-value store
std::fs::OpenOptions	append mode for the log
std::fs::File::create	truncate and rewrite during compaction
std::fs::read_to_string	load the log on startup
writeln!	write entries to the log
splitn(2, ' ')	parse log lines with values that may contain spaces
String vs &str	owned vs borrowed strings when inserting into HashMap
Ownership & move semantics	why append must be called before store.insert
Design Inspiration

This project implements the core ideas behind Bitcask — the storage engine used by the Riak database:

Append-only log for durability
In-memory index (HashMap) for fast reads
Periodic compaction to reclaim disk space

Real production databases add more on top: write-ahead logs, checksums, bloom filters, and multi-file segments. But the core idea is exactly what you built here.

Ideas for Extending
 TTL support — SET key value 3600 expires the key after 1 hour
 Transactions — BEGIN, COMMIT, ROLLBACK
 Binary file format — store data as bytes instead of plain text for speed
 HTTP API — expose the store over HTTP so any language can use it
 Checksums — detect corrupted log entries on load
 Multiple log segments — split the log into files for easier compaction
 Bloom filter — skip disk reads for keys that definitely don't exist
 Benchmarking — measure reads/writes per second
What I Learned
How append-only logs make writes fast and safe
Why reads stay fast even with a large log — the HashMap is the index
How compaction works and why databases need it
Rust ownership in practice — why append must come before store.insert
How real storage engines like Bitcask and LevelDB are designed
OpenOptions for fine-grained file control (append vs truncate)
License

MIT