use rusqlite::{params, Connection, types::Type};

pub fn table_exists(name: &str, conn: &Connection) -> bool {
    let sql: &str = "SELECT count() count FROM sqlite_master WHERE name = ?";

    return conn
        .query_row(sql, params![name], |row| row.get(0))
        .unwrap();
}

pub fn print_persons_table(conn: &Connection) -> Result<(), rusqlite::Error> {
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
    let lenght_query = "SELECT MAX(LENGTH(id)), MAX(LENGTH(name)), MAX(LENGTH(age)), MAX(LENGTH(occupationId)) FROM Persons";
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

    println!(); // Spacing.
    // Join col_names together for form a header line with format.
    for i in 0..col_names.len() {
        col_names[i] = format!("{:<width$}", col_names[i], width=max_lenghts[i] as usize);
    }

    let header_string = col_names.join(" | ");
    let header_string_col_spacers = find_indexes(&header_string, '|');
    println!("{}", header_string);
    // Print line.
    let mut spacer_line = format!("{:-<width$}", "-", width=header_string.len());
    for space in header_string_col_spacers {
        spacer_line.replace_range(space..space + 1, "+");
    }
    println!("{}", spacer_line);
    // Print table_content
    println!("{}", table_content);

    Ok(())
}

pub fn find_indexes(s: &String, c: char) -> Vec<usize> {
    let mut indexes = Vec::new();
    let mut pos = 0;
    while let Some(i) = s[pos..].find(c) {
        pos += i;
        indexes.push(pos);
        pos += 1;
    }
    indexes
}