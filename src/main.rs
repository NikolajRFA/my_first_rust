use rusqlite::{params, Connection};
use std::{io, process, /*io::Result*/};

fn main() {
    // Print instructions.
    println!(); // Console spacing.
    println!("What's your name and age? (Seperated by';')");
    println!("You can enter multiple persons by seperating with ','!");
    println!("\nExit the program by typing 'exit'");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Exit program.
    if input.trim().eq(&String::from("exit")) {
        process::exit(0);
    }

    // Print persons table.
    if input.trim().eq(&String::from("print persons")) {
        print_persons_table(&Connection::open("name_database.db").unwrap()).unwrap();
        process::exit(0);
    }

    let name_age_sets: Vec<&str> = input.trim().split(",").collect();

    for set in name_age_sets {
        let name_age: Vec<&str> = set.trim().split(";").collect();

        let name = name_age[0].trim();
        let age = name_age[1].trim();
        let age_int: i16 = age.parse().unwrap();

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
        if insert_person(&conn, name, age_int) {
            println!("{} was inserted!", name);
        } else {
            println!("Something went wrong while inserting {}!", name);
        }
    }
}

fn table_exists(name: &str, conn: &Connection) -> bool {
    let sql: &str = "SELECT count() count FROM sqlite_master WHERE name = ?";

    return conn
        .query_row(sql, params![name], |row| row.get(0))
        .unwrap();
}

fn insert_person(conn: &Connection, name: &str, age: i16) -> bool {
    let sql: &str = "INSERT INTO Persons (name, age) VALUES (?1, ?2)";

    match conn.execute(sql, params![name, age]) {
        Ok(_) => return true,
        Err(err) => {
            println!("Insert failed!\nError msg: {}", err);
            return false;
        }
    }
}

fn print_persons_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    let query = "PRAGMA table_info(Persons)";
    let mut stmt = conn.prepare(query)?;
    let row_iter = stmt.query_map(params![], |row| {
        let name = row.get_ref::<&str>("name")?.as_str()?;
        let name = name.to_owned();
        Ok(name)
    })?;

    for row in row_iter {
        match row {
            Ok(name) => {
                print!("{} ", name);
            }
            Err(e) => {
                println!("Error in retrieving data from row: {}", e);
            }
        }
    }

    return Ok(());
}
