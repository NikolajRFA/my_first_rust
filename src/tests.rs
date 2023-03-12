#[cfg(test)]
mod tests {
    use rusqlite::{params, Connection};
    use crate::{person::{Person}};
    
    #[test]
    fn person_insertion() {
        let conn = Connection::open_in_memory().unwrap();
        let insertion = Person::insert_person(&conn, "Adam", 23);

        assert!(insertion);
    }

    #[test]
    fn person_insertion_from_person() {
        let conn = Connection::open_in_memory().unwrap();
        let person = Person {id: 1, name: String::from("Adam"), age: 23, occupation_id: None};
        let insertion = Person::insert_from_person(&conn, &person);

        assert!(insertion);
    }

    #[test]
    fn get_person_from_id() {
        let conn = Connection::open_in_memory().unwrap();
        Person::insert_person(&conn, "Adam", 23);

        let person = Person::get_from_id(1, &conn).unwrap();

        assert_eq!(person, Person {id: 1, name: String::from("Adam"), age: 23, occupation_id: None});
    }
}


