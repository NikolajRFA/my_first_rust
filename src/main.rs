use rusqlite::{params, Connection};
use std::{io, process};

fn main() {
    // Print instructions.
    println!(); // Console spacing.
    println!("What's your name and age? (Seperated by';')");
    println!("You can enter multiple persons by seperating with ','!");
    println!("\nExit the program by typing 'exit'");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if input.trim().eq(&String::from("exit")) {
        process::exit(0);
    }

    let name_age_sets: Vec<&str> = input.trim().split(",").collect();

    for set in name_age_sets {
        let name_age: Vec<&str> = set.trim().split(";").collect();

        let name = name_age[0].trim();
        let age = name_age[1].trim();
        let age_int: i8 = age.parse().unwrap();

        println!("Hello, {}!", name);

        let conn = Connection::open("name_database.db").unwrap();

        // Check if Persons table exists
        if !table_exists("Persons", &conn) {
            // Create Persons table.
            conn.execute(
                "CREATE TABLE Persons (
                    id              INTEGER PRIMARY KEY AUTOINCREMENT,
                    name            TEXT NOT NULL,
                    age             INTEGER NULL
                    )",
                params![],
            )
            .unwrap();
        }

        // Insert person into table.
        insert_person(&conn, name, age_int);

        let selected_name: String = conn
            .query_row(
                "SELECT name FROM Persons WHERE id = (SELECT MAX(id) FROM Persons)",
                params![],
                |row| row.get(0),
            )
            .unwrap();

        println!("{} has been added!", selected_name);
    }
}

fn table_exists(name: &str, conn: &Connection) -> bool {
    let sql: &str = "SELECT count() count FROM sqlite_master WHERE name = ?";

    return conn
        .query_row(sql, params![name], |row| row.get(0))
        .unwrap();
}

fn insert_person(conn: &Connection, name: &str, age: i8) -> bool {
    let sql: &str = "INSERT INTO Persons (name, age) VALUES (?1, ?2)";

    match conn.execute(sql, params![name, age]) {
        Ok(_) => return true,
        Err(err) => {
            println!("Insert failed!\nError msg: {}", err);
            return false;
        }
    }
}
