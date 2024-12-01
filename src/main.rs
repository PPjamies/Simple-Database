mod data;
mod datastore;
mod log_handler;
mod compact;

use data::Data;
use datastore::{DBConfig, DBStore};

fn db_init() -> DBStore {
    let segments = vec![
        String::from("<path-to-first-segment>"),
        String::from("<path-to-second-segment>"),
    ];

    let db_config: DBConfig = DBConfig::new(
        String::from("Simple Datastore"),
        String::from("<path-to-log-file>"),
        segments,
        String::from("<path-to-crash-recovery-file>"),
    );

    DBStore::new(db_config)
}

fn main() {
    println!("Welcome to Simple DataStore!");

    let db: DBStore = db_init();

    println!("Enter key:");
    let mut key = String::new();
    std::io::stdin()
        .read_line(&mut key)
        .expect("Failed to read key");

    println!("Enter value:");
    let mut val = String::new();
    std::io::stdin()
        .read_line(&mut val)
        .expect("Failed to read value");

    let data: Data = Data::new(key, val);

    println!("Key and value stored!");
}
