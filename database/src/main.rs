use anyhow::Error;
use rusqlite::{Connection, Result};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn load_table_definitions() -> Result<Vec<String>, anyhow::Error> {
    // https://stackoverflow.com/questions/46749360/how-to-get-only-the-directory-portion-of-the-current-executables-path
    let toml_dir = env!("CARGO_MANIFEST_DIR");
    let mut table_dir = PathBuf::from(String::from(toml_dir));
    table_dir.extend(&["src", "table_definitions"]);
    let mut tables: Vec<String> = Vec::<String>::new();
    for entry in fs::read_dir(table_dir)? {
        let entry = entry?;
        let path = entry.path();
        let sql_def: String = fs::read_to_string(path)?;
        tables.push(sql_def);
    }
    Ok(tables)
}

fn main() -> Result<()> {
    let conn = Connection::open_in_memory()?;
    // load and run table definitions
    let table_defs = match load_table_definitions() {
        Ok(table_defs) => table_defs,
        Err(error) => panic!("Unable to load database definitions: {error:?}"),
    };
    for table_def in table_defs.iter() {
        println!("table def: {}", table_def);
        conn.execute(
            table_def.as_str(),
            (), // empty list of parameters.
        )?;
    }
    let me = Person {
        id: 0,
        name: "Steven".to_string(),
        data: None,
    };
    conn.execute(
        "INSERT INTO person (name, data) VALUES (?1, ?2)",
        (&me.name, &me.data),
    )?;

    let mut stmt = conn.prepare("SELECT id, name, data FROM person")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Person {
            id: row.get(0)?,
            name: row.get(1)?,
            data: row.get(2)?,
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }
    Ok(())
}
