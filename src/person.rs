use rusqlite::{params, Connection, OptionalExtension};

#[derive(Debug)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub age: i16,
    pub occupation_id: Option<i16>,
}

impl Person {
    pub fn get_person_from_id(id: i64, conn: &Connection) -> Option<Self> {
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
}