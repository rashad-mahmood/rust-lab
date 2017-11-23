extern crate csv;
extern crate rusqlite;

use rusqlite::Connection;
use std::fs::File;
use std::error::Error;

fn main() {
    println!("Hello, world!");
    test();
}

fn test() -> Result<(), Box<Error>> {
    let f = File::open("accountactivity.csv")?;
    let mut reader = csv::Reader::from_reader(f);
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn create_table() {
    let conn = Connection::open_in_memory().unwrap();

    conn.execute("CREATE TABLE transaction (
                                           id INTEGER PRIMARY KEY,
                                           name TEXT NOT NULL,
                                           date TEXT NOT NULL,
                                           amount REAL NOT NULL,
                                           type TEXT NOT NULL,
                                           balance REAL
                                           )", &[]).unwrap()
}
