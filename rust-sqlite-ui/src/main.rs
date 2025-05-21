#![allow(dead_code)]
use rusqlite::{ Connection, Result };

#[derive(Debug)]
struct Person {
    id: i32,
    name: String,
    data: Option<Vec<u8>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // -- Memory SQLite
    let conn = Connection::open_in_memory()?;
    // -- Create Schema
    conn.execute(
        "CREATE TABLE person (
            id   INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            data BLOB
        ) STRICT",
        ()
    )?;
    // -- Insert
    // OK in `strict` mode
    let p = Person {
        id: 0,
        name: "Jean".to_string(),
        data: None,
    };
    conn.execute("INSERT INTO person(name,data)
        VALUES(?1,?2)", (&p.name, &p.data))?;

    // -- Select
    let mut stmt = conn.prepare("
    SELECT id, name, data FROM person")?;
    let mut rows = stmt.query([])?;

    while let Some(row) = rows.next()? {
        let name: String = row.get(1)?;
        println!("-->> name: {}", name);
        println!("-->> row: {:?}", row);
    }

    // -- Print table
    pretty_sqlite::print_table(&conn, "person")?;

    Ok(())
}
