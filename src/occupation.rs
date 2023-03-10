use rusqlite::{params, Connection, OptionalExtension};
use crate::person::Person;

#[derive(Debug)]
pub struct Occupation {
    pub id: i64,
    pub trade: String,
}

impl Occupation {
    pub fn get_occupation_from_person(person: &Person, conn: &Connection) -> Option<Self> {
        let sql = "SELECT * FROM Occupations WHERE id = ?";
        let occupation = conn.query_row(sql, params![person.occupation_id], |row| {
            let occupation = Occupation {
                id: row.get("id")?,
                trade: row.get("trade")?,
            };
            Ok(occupation)
        }).optional().unwrap_or(None);
        // Return
        occupation
    }
}