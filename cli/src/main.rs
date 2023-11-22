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
        match prepare_db(&mut conn) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        };
        match ttags_create(&mut conn, path) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        };
    }
}
