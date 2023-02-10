use rusqlite::{params, Connection};
use std::io;

fn main() {
    println!("What's your name and age? (Seperated by';')");

    let mut name_age_in = String::new();
    io::stdin().read_line(&mut name_age_in).expect("Failed to read line");

    let name_age: Vec<&str> = name_age_in.trim().split(";").collect();
    
    println!("Hello, {}!", name_age[0]);

    let conn = Connection::open("name_database.db").unwrap();

    conn.execute(
        "CREATE TABLE Persons (
                  id              INTEGER PRIMARY KEY AUTOINCREMENT,
                  name            TEXT NOT NULL,
                  age             INTEGER NULL
                  )",
        params![],
    )
    .unwrap();

    conn.execute(
        "INSERT INTO Persons (name, age) VALUES (?1, ?2)",
        params![name_age[0].trim(), name_age[1].trim()],
    )
    .unwrap();

    let selected_name: String = conn
        .query_row("SELECT name FROM Persons WHERE id = (SELECT MAX(id) FROM Persons)", params![], |row| {
            row.get(0)
        })
        .unwrap();

    println!("The name you entered was: {}", selected_name);
}