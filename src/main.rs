use sqlite::{self, OpenFlags};
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Write;

/*
 * Usage:
 * Set a value (in the future, the value will time out after 1 hour):
 * kvcache set key value
 *
 * Get a value:
 * kvcache get key
 *
 * Get a value if we have it, otherwise run a command and store the result:
 * kvcache try key command
 *
 */

// TODO: Find home folder
const DB_PATH: &str = "file:///Users/peder/kv.db";

// TODO: Remove?
const LOG_PATH: &str = "/Users/peder/kvcache.log";

fn main() {
    init();

    // The first arg is the relative path of the program
    let args: Vec<String> = env::args().collect();

    // println!("{:?}", args);

    // Append args to log file
    let mut file: File = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(LOG_PATH)
        .unwrap();

    // TODO: Add timestamp to log message
    file.write_all(format!("{:?}\n", args).as_bytes()).unwrap();

    if args.len() < 3 {
        println!("Not enough arguments");
        return;
    }

    match args[1].as_str() {
        "set" => {
            let key = &args[2];
            let value = &args[3];
            insert_kv(key, value);
        }
        "get" => {
            let key = &args[2];
            let value = read_kv(key);

            match value {
                Some(v) => {
                    println!("{}", v);
                }
                None => {
                    println!("No value found for key {}", key);
                }
            }
        }
        "try" => {
            let key = &args[2];
            let value = read_kv(key);

            match value {
                Some(v) => {
                    println!("{}", v);
                }
                None => {
                    let command = &args[3];
                    let output = std::process::Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .output()
                        .expect("Failed to execute command");

                    let output_string = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    insert_kv(key, &output_string);
                    println!("{}", output_string);
                }
            }
        }
        _ => {
            println!("Invalid command");
        }
    }
}

// Only conditionally create the table if it doesn't exist
fn init() {
    let opts = OpenFlags::new().with_uri().with_read_write().with_create();
    let connection = sqlite::Connection::open_with_flags(DB_PATH, opts).unwrap();

    // TODO: Add support for time-to-live
    let query = "CREATE TABLE IF NOT EXISTS kv (key TEXT, value TEXT);";

    connection.execute(query).unwrap();
}

fn insert_kv(key: &str, value: &str) {
    let opts = OpenFlags::new().with_uri().with_read_write().with_create();

    let connection =
        sqlite::Connection::open_with_flags("file:///Users/peder/kv.db", opts).unwrap();
    let query = format!("INSERT INTO kv VALUES ('{}', '{}');", key, value);
    connection.execute(query).unwrap();
}

fn read_kv(key: &str) -> Option<std::string::String> {
    let opts = OpenFlags::new().with_uri().with_read_write().with_create();

    let connection =
        sqlite::Connection::open_with_flags("file:///Users/peder/kv.db", opts).unwrap();
    let query = "SELECT * FROM kv WHERE key = ?";

    let foobie: Vec<String> = connection
        .prepare(query)
        .unwrap()
        .into_iter()
        .bind((1, key))
        .unwrap()
        .map(|row| row.unwrap())
        .map(|row| String::from(row.read::<&str, _>("value")))
        .collect();

    // Check single item
    let single_element_or_nothing = foobie.first().map(|x| x.clone());
    return single_element_or_nothing;
}
