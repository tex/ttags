use ttags::*;
use clap::Parser;
use easy_parallel::Parallel;

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

pub fn ttags_complete(symbol: &str) -> Result<(), rusqlite::Error> {
    Parallel::new()
        .each(globwalk::glob(".ttags.*.db").expect("Error when searching for .ttags.*.db files"), |db| {
            let query = "SELECT DISTINCT name FROM tags WHERE is_definition=true AND name LIKE ?1".to_string();
            let conn = rusqlite::Connection::open(db.unwrap().path()).expect("Error opening in-memory database");
            let mut stmt = conn.prepare(&query).unwrap();
            let mut rows = stmt.query(rusqlite::params![format!("{}%", symbol)]).unwrap();
            while let Some(row) = rows.next().unwrap() {
                println!("{}",
                    row.get::<_, String>(0).unwrap());  // name
            }
        })
        .run();
    Ok(())
}

fn ttags_find(is_definition: bool, symbol: &str) -> Result<(), rusqlite::Error> {
    Parallel::new()
        .each(globwalk::glob(".ttags.*.db").expect("Error when searching for .ttags.*.db files"), |db| {
            let query = "SELECT DISTINCT file,name,row FROM tags WHERE is_definition=?1 AND name GLOB ?2".to_string();
            let conn = rusqlite::Connection::open(db.unwrap().path()).expect("Error opening in-memory database");
            let mut stmt = conn.prepare(&query).unwrap();
            let mut rows = stmt.query(rusqlite::params![is_definition, symbol]).unwrap();
            while let Some(row) = rows.next().unwrap() {
                println!("{}:{}:{}",
                    row.get::<_, String>(0).unwrap(),   // file
                    row.get::<_, usize>(2).unwrap(),    // row
                    row.get::<_, String>(1).unwrap());  // name
            }
        })
        .run();
    Ok(())
}

fn main()  {
    let cli = Cli::parse();

    if let Some(symbol) = cli.reference.as_deref() {
        ttags_find(false, symbol).expect("Find reference failed");
    } else if let Some(symbol) = cli.definition.as_deref() {
        ttags_find(true, symbol).expect("Find definition failed");
    } else if let Some(symbol) = cli.complete.as_deref() {
        ttags_complete(symbol).expect("Find completion failed");
    } else {
        let path: &str = if let Some(p) = cli.complete.as_deref() { p } else { "." };
        ttags_create(path);
    }
}
