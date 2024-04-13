use ttags::*;
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
    let mut query = "SELECT DISTINCT name FROM db0.tags WHERE is_definition=true AND name LIKE ?1".to_string();
    for_each_db(|_path, index| {
        if index != 0 {
            query.push_str(
                &format!(" UNION SELECT DISTINCT name FROM db{}.tags WHERE is_definition=true AND name LIKE ?1", index));
        }
    });
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(rusqlite::params![format!("{}%", symbol)])?;
    while let Some(row) = rows.next()? {
        println!("{}",
            row.get::<_, String>(0)?);  // name
    }
    Ok(())
}

fn ttags_find(conn: &mut rusqlite::Connection, is_definition: bool, symbol: &str) -> Result<(), rusqlite::Error> {
    let mut query = "SELECT DISTINCT file,name,row FROM db0.tags WHERE is_definition=?1 AND name GLOB ?2".to_string();
    for_each_db(|_path, index| {
        if index != 0 {
            query.push_str(
                &format!(" UNION ALL SELECT DISTINCT file,name,row FROM db{}.tags WHERE is_definition=?1 AND name GLOB ?2", index));
        }
    });
    let mut stmt = conn.prepare(&query)?;
    let mut rows = stmt.query(rusqlite::params![is_definition, symbol])?;
    while let Some(row) = rows.next()? {
        println!("{}:{}:{}",
            row.get::<_, String>(0)?,   // file
            row.get::<_, usize>(2)?,    // row
            row.get::<_, String>(1)?);  // name
    }
    Ok(())
}

fn for_each_db<F>(mut f: F) where F: FnMut(&std::path::Path, usize) {
    let mut index : usize = 0;
    for file in globwalk::glob(".ttags.*.db").expect("Error when searching for .ttags.*.db files") {
        if let Ok(file) = file {
            f(file.path(), index);
            index += 1;
        }
    }
}

fn open_db() -> rusqlite::Connection {
    let conn = rusqlite::Connection::open(":memory:").expect("Error opening in-memory database");
    for_each_db(|file, index| {
        conn.execute(&format!("ATTACH \"{}\" as db{};", file.to_string_lossy(), index), [])
            .expect(&format!("Attaching database ({}) failed", file.to_string_lossy()));
    });
    conn
}

fn main()  {
    let cli = Cli::parse();
    let mut conn = open_db();

    if let Some(symbol) = cli.reference.as_deref() {
        ttags_find(&mut conn, false, symbol).expect("Find reference failed");
    } else if let Some(symbol) = cli.definition.as_deref() {
        ttags_find(&mut conn, true, symbol).expect("Find definition failed");
    } else if let Some(symbol) = cli.complete.as_deref() {
        ttags_complete(&mut conn, symbol).expect("Find completion failed");
    } else {
        let path: &str = if let Some(p) = cli.complete.as_deref() { p } else { "." };
        ttags_create(path);
    }
}
