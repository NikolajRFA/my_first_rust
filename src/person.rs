use rusqlite::{params, Connection, OptionalExtension};
use crate::functions::table_exists;

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub age: i16,
    pub occupation_id: Option<i16>,
}

impl Person {
    pub fn get_from_id(id: i64, conn: &Connection) -> Option<Self> {
        // Retrieve data from SQL.
        let sql = "SELECT * FROM Persons WHERE id = ?";
        let person = conn.query_row(sql, params![id], |row| {
            let person = Person {
                id: row.get("id")?,
                name: row.get("name")?,
                age: row.get("age")?,
                occupation_id: row.get("occupationId")?,
            };
            Ok(person)
        }).optional().unwrap_or(None);
        // Create and retrun Person struct.
        person
    }

    pub fn insert_person(conn: &Connection, name: &str, age: i16) -> bool {   
        create_persons_table(&conn); 
        let sql = "INSERT INTO Persons (name, age) VALUES (?1, ?2)";
    
        match conn.execute(sql, params![name, age]) {
            Ok(_) => return true,
            Err(err) => {
                println!("Insert failed!\nError msg: {}", err);
                return false;
            }
        }
    }

    pub fn insert_from_person(conn: &Connection, person: &Person) -> bool {
        create_persons_table(&conn);

        let sql = "INSERT INTO Persons (name, age, occupationId) VALUES (?1, ?2, ?3)";

        match conn.execute(sql, params![person.name, person.age, person.occupation_id]) {
            Ok(_) => return true,
            Err(err) => {
                println!("Insert failed!\nError msg: {}", err);
                return false;
            }
        }
    }
}

fn create_persons_table(conn: &Connection) {
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
}
