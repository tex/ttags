use clap::Parser;
use std::collections::HashMap;

use tree_sitter_tags::TagsContext;
use tree_sitter_tags::TagsConfiguration;

use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use std::thread;

const PATTERNS: [&str; 9] = [ "*.rs", "*.cpp", "*.hpp", "*.cc", "*.hh", "*.c", "*.h", "*.py", "*.js" ];

#[derive(Debug)]
struct Entry {
    file: String,
    name: String,
    is_definition: bool,
    syntax_type_id: u32,
    row: usize,
    column: usize,
}

fn parse(file: &globwalk::DirEntry, conf: &TagsConfiguration) -> Vec<Entry> {
    let mut context = TagsContext::new();
    let code = std::fs::read(file.path()).unwrap();
    let tags = context.generate_tags(
        &conf,
        &code,
        None,
    ).unwrap().0;

    let mut res: Vec<Entry> = Vec::new();

    for tag in tags {
        let tag: tree_sitter_tags::Tag = tag.unwrap();
        let entry: Entry = Entry {
            file: file.path().to_str().unwrap_or("").to_string(),
            name: std::str::from_utf8(&code[tag.name_range]).unwrap_or("").to_string(),
            is_definition: tag.is_definition,
            syntax_type_id: tag.syntax_type_id,
            row: tag.span.start.row + 1,
            column: tag.span.start.column + 1,
        };
        res.push(entry);
    };

    return res;
}

// tags - all tags retrieved from one file
fn process(tags: Vec<Entry>, conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    if tags.len() == 0 {
        return Ok(());
    }
    let tx = conn.transaction()?;
    tx.execute("delete from tags where file = (?1)", [&tags.get(0).unwrap().file])?;
    for tag in tags {
        tx.execute("insert into tags (file, name, is_definition, syntax_type_id, row, column) values (?1, ?2, ?3, ?4, ?5, ?6)",
            [tag.file,
                tag.name,
                tag.is_definition.to_string(),
                tag.syntax_type_id.to_string(),
                tag.row.to_string(),
                tag.column.to_string()])?;
    }
    tx.commit()?;
    Ok(())
}

fn scan(sw: Sender<globwalk::DirEntry>, rr: Receiver<Vec<Entry>>, conn: &mut rusqlite::Connection, path: &str) -> Result<(), globwalk::GlobError> {
    println!("scan begin");
    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        path, // BASE_DIR,
        &PATTERNS,
    )
    .follow_links(true)
    .case_insensitive(true)
    .build()?
    .into_iter()
    .filter_map(Result::ok);

    let mut count = 0;

    println!("scan end");
    for file in walker {
        match sw.send(file) {
            Ok(_) => count += 1,
            Err(e) => println!("Error: {}", e)
        };

        for tags in rr.try_iter() {
            count -= 1;
            match process(tags, conn) {
                Ok(_) => continue,
                Err(e) => {
                    println!("Error {}", e);
                    return Ok(());
                }
            };
        }
    }
    println!("Finished adding");
    for tags in rr.iter() {
        count -= 1;
        match process(tags, conn) {
            Ok(_) => true,
            Err(e) => {
                println!("Error {}", e);
                false
            }
        };
        if count == 0 {
            break;
        };
    }

    return Ok(());
}

// read_to_string can be replaced with include_str!
// former reads it in run-time, latter reads it in compile-time
fn create_configuration() -> Result<HashMap<&'static str, TagsConfiguration>, tree_sitter_tags::Error> {
    let mut conf = HashMap::new();

    conf.insert("rs",
        TagsConfiguration::new(
            tree_sitter_rust::language(),
            &std::fs::read_to_string("./src/tags_rust.scm")
                .expect("Error in reading file"),
            "")?);

    let cpp = || TagsConfiguration::new(
        tree_sitter_cpp::language(),
        &std::fs::read_to_string("./src/tags_cpp.scm")
            .expect("Error in reading file"),
        "");
    conf.insert("cc", cpp()?);
    conf.insert("hh", cpp()?);
    conf.insert("cpp", cpp()?);
    conf.insert("hpp", cpp()?);

    let c = || TagsConfiguration::new(
        tree_sitter_c::language(),
        include_str!("tags_c.scm"),
        "");
    conf.insert("c", c()?);
    conf.insert("h", c()?);

    conf.insert("js",
        TagsConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::TAGGING_QUERY,
            tree_sitter_javascript::LOCALS_QUERY)?);

    conf.insert("py",
        TagsConfiguration::new(
            tree_sitter_python::language(),
            tree_sitter_python::TAGGING_QUERY,
            "")?);

    return Ok(conf);
}

// worker needs a channel to receive commands to workers
// and to send results from workers
fn worker(i: usize, rw: Receiver<globwalk::DirEntry>, sr: Sender<Vec<Entry>>) {
    println!("worker {}", i);
    let conf = match create_configuration() {
        Ok(conf) => conf,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };
    println!("ready {}", i);
    for dir_entry in rw.iter() {
        println!("Thread {} got a job! {}", i, dir_entry.path().display());
        let ext = match dir_entry.path().extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => ext,
            _ => continue,
        };
        let tagsconf = match conf.get(ext) {
            Some(tagsconf) => tagsconf,
            _ => continue,
        };

        let res = parse(&dir_entry, &tagsconf);
        match sr.send(res) {
            Ok(_) => continue,
            Err(_) => break,
        };
    }
}

fn prepare_db(conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "create table if not exists tags (
            id integer primary key,
            file text not null,
            name text not null,
            is_definition bool,
            syntax_type_id integer,
            row integer,
            column integer
         )",
        [],
    )?;
    conn.execute(
        "create index if not exists idx_tags on tags(name, is_definition)",
        [],
    )?;
    Ok(())
}

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

fn ttags_complete(conn: &mut rusqlite::Connection, symbol: &str) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT DISTINCT name FROM tags WHERE is_definition=? AND name GLOB ?")?;
    let mut rows = stmt.query(["true".to_string(), format!("{}", symbol)])?;
    while let Some(row) = rows.next()? {
        println!("{}",
            row.get::<_, String>(0)?);  // name
    }
    return Ok(());
}

fn ttags_find(conn: &mut rusqlite::Connection, is_definition: bool, symbol: &str) -> Result<(), rusqlite::Error> {
    let mut stmt = conn.prepare(
        "SELECT DISTINCT file, name, is_definition, syntax_type_id, row, column FROM tags WHERE is_definition=? AND name GLOB ?")?;
    let mut rows = stmt.query([
        format!("{}", if is_definition { "true" } else { "false" }),
        format!("{}", symbol)])?;
    while let Some(row) = rows.next()? {
        println!("{}:{}:{}",
            row.get::<_, String>(0)?,   // file
            row.get::<_, usize>(4)?,    // row
            row.get::<_, String>(1)?);  // name
    }
    return Ok(());
}

fn ttags_create(conn: &mut rusqlite::Connection, path: &str) -> Result<(), globwalk::GlobError> {
    // channel for giving commands to workers
    let (sw, rw) = bounded(num_cpus::get() * 5);
    // channel for reporting results from workers
    let (sr, rr) = unbounded();

    for i in 0..num_cpus::get() {
        let rw = rw.clone();
        let sr = sr.clone();
        thread::spawn(move || {
            worker(i, rw, sr);
        });
    };

    // scanner needs to be able to give commands to workers (sw)
    // and to retrieve results from workers (rr)
    return scan(sw, rr, conn, path);
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
    match prepare_db(&mut conn) {
        Ok(_) => (),
        Err(e) => println!("Error: {}", e),
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
        let p : &str = if let Some(path) = cli.complete.as_deref() { path } else { "." };
        match ttags_create(&mut conn, p) {
            Ok(_) => (),
            Err(e) => println!("Error: {}", e),
        };
    }
}
