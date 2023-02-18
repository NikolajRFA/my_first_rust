use rusqlite::{params, Connection, types::{Type}};
use std::{io, process, env};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
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
    // TODO: Once done do abstraction of function to be generic.
    // Query pragma table info.
    let pragma_query = "PRAGMA table_info(Persons)";
    let mut pragma_stmt = conn.prepare(pragma_query)?;

    let pragma_row_iter = pragma_stmt.query_map(params![], |row| {
        let name = row.get_ref("name")?.as_str()?;
        let name = name.to_owned();
        Ok(name)
    })?;

    // Create vector of column names.
    let mut col_names = Vec::new();
    for row in pragma_row_iter {
        match row {
            Ok(name) => {
                col_names.push(name);
            }
            Err(e) => {
                eprintln!("Error in retrieving data from row: {}", e);
            }
        }
    }

    // Get max lenghts.
    let lenght_query = "SELECT MAX(LENGTH(id)), MAX(LENGTH(name)), MAX(LENGTH(age)) FROM Persons";
    let max_lenghts = conn.query_row(lenght_query, [],|row| {
        let mut max_lenghts_internal = Vec::new();
        for i in 0..col_names.len() {
            let length = match row.get_ref(i)? {
                rusqlite::types::ValueRef::Null => 0,
                value_ref => value_ref.as_i64()?,
            };
            if length >= col_names[i].len().try_into().unwrap() {
                max_lenghts_internal.push(length);
            } else {
                max_lenghts_internal.push(col_names[i].len().try_into().unwrap());
            }
            
        }
        Ok(max_lenghts_internal)
    })?;

    // Get the actual columns of the table.
    let data_query = "SELECT * FROM Persons";
    let mut data_stmt = conn.prepare(data_query)?;

    // Query data rows
    let mut data_rows = data_stmt.query(params![])?;
    // Initialize content String.
    let mut table_content = String::from("");
    // Iterate over the data rows.
    while let Some(row) = data_rows.next()? {
        // Initialize a vector for the content of the row.
        let mut row_content = Vec::new();

        // Iterate over the columns of the row.
        for col in 0..col_names.len() {

            let value_ref = row.get_ref(col)?;
            let data_type = value_ref.data_type();
            // Match datatype.
            let col_value = match data_type {
                Type::Null => 
                    format!("{:<width$}", "NULL", width=max_lenghts[col] as usize).as_str().to_owned(),
                Type::Integer => 
                    format!("{:<width$}", value_ref.as_i64()?, width=max_lenghts[col] as usize).as_str().to_owned(),
                Type::Real => 
                    format!("{:<width$}", value_ref.as_f64()?, width=max_lenghts[col] as usize).as_str().to_owned(),
                Type::Text => 
                    format!("{:<width$}", value_ref.as_str()?, width=max_lenghts[col] as usize).as_str().to_owned(),
                Type::Blob => 
                    format!("{:<width$?}", value_ref.as_blob()?, width=max_lenghts[col] as usize).as_str().to_owned()
            };
            // Push column value to row_content vector
            row_content.push(col_value);
        }
        // Join row_content
        table_content.push_str(&row_content.join(" | "));
        // Push breakline.
        table_content.push_str("\n");
    }

    // OUTPUT:

    // Join col_names together for form a header line with format.
    for i in 0..col_names.len() {
        col_names[i] = format!("{:<width$}", col_names[i], width=max_lenghts[i] as usize);
    }

    let header_string = col_names.join(" | ");
    println!("{}", header_string);
    // Print line.
    println!("{:-<width$}", "-", width=header_string.len());
    // Print table_content
    println!("{}", table_content);

    return Ok(());
}
