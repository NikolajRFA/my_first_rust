mod person;
mod occupation;
mod functions;
mod tests;

use rusqlite::{Connection};
use std::{io, process, env};
use crate::{person::Person, occupation::Occupation, functions::{print_persons_table}};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    // Print instructions.
    println!(); // Console spacing.

    loop {
        println!("What's your name and age? (Seperated by ';')");
        println!("You can enter multiple persons by seperating with ','!");
        println!("\nExit the program by typing 'exit'");

        // TODO: Handle invalid input.
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Exit program.
        if input.trim().eq(&String::from("exit")) {
            process::exit(0);
        }

        // Print persons table.
        if input.trim().eq(&String::from("print persons")) {
            print_persons_table(&Connection::open("name_database.db").unwrap()).unwrap();
            continue;
        }

        if input.trim().eq(&String::from("get person with id")) {
            // TODO: Add handling for invalid input.
            println!("Enter id:");
            let mut person_id = String::new();
            io::stdin().read_line(&mut person_id).expect("Failed to read line");

            // Get person.
            let conn = Connection::open("name_database.db").unwrap();
            let person = Person::get_from_id(person_id.trim().parse().unwrap(), &conn).unwrap();

            
            // Print person details.
            println!(
                "Person with id: {} has name {} and age {}",
                person.id,
                person.name,
                person.age
            );

            match Occupation::get_occupation_from_person(&person, &conn) {
                None => println!("Person has no occupation."),
                Some(occupation) => {
                    println!("Person has trade {}", occupation.trade);
                }
            }

            println!(); // Extra spacing.

            // Continue next run of loop.
            continue;
        }

        let name_age_sets: Vec<&str> = input.trim().split(",").collect();

        for set in name_age_sets {
            let name_age: Vec<&str> = set.trim().split(";").collect();

            let name = name_age[0].trim();
            let age = name_age[1].trim();
            let age_int: i16 = age.parse().unwrap();

            let conn = Connection::open("name_database.db").unwrap();

            // Insert person into table.
            if Person::insert_person(&conn, name, age_int) {
                println!("{} was inserted!", name);
            } else {
                println!("Something went wrong while inserting {}!", name);
            }
        }
    }
}
