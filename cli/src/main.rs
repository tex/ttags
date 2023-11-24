extern crate lib;

use lib::module_ttags::*;
use clap::Parser;

/// By default a tag database is created for current folder recursively
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Find references
    #[clap(long, short)]
    reference: Option<String>,

    /// Find definitions
    #[clap(long, short)]
    definition: Option<String>,

    /// Complete symbols
    #[clap(short)]
    complete: Option<String>,

    /// Path to scan
    #[clap(long, short)]
    path: Option<String>,
}

pub fn ttags_complete(conn: &mut rusqlite::Connection, symbol: &str) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT DISTINCT name FROM tags WHERE is_definition=?1 AND name GLOB ?2")?;
    let mut rows = stmt.query(rusqlite::params![true, symbol])?;
    while let Some(row) = rows.next()? {
        println!("{}",
            row.get::<_, String>(0)?);  // name
    }
    return Ok(());
}

fn ttags_find(conn: &mut rusqlite::Connection, is_definition: bool, symbol: &str) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT file,name,row FROM tags WHERE is_definition=?1 AND name GLOB ?2")?;
    let mut rows = stmt.query(rusqlite::params![is_definition, symbol])?;
    while let Some(row) = rows.next()? {
        println!("{}:{}:{}",
            row.get::<_, String>(0)?,   // file
            row.get::<_, usize>(2)?,    // row
            row.get::<_, String>(1)?);  // name
    }
    return Ok(());
}

fn main()  {
    let cli = Cli::parse();

    let mut conn = match rusqlite::Connection::open("ttags.db") {
        Ok(conn) => conn,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    if let Some(symbol) = cli.reference.as_deref() {
        match ttags_find(&mut conn, false, symbol) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        };
    } else if let Some(symbol) = cli.definition.as_deref() {
        match ttags_find(&mut conn, true, symbol) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        };
    } else if let Some(symbol) = cli.complete.as_deref() {
        match ttags_complete(&mut conn, symbol) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        };
    } else {
        let path : &str = if let Some(p) = cli.complete.as_deref() { p } else { "." };
        match ttags_create(&mut conn, path) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        };
    }
}
