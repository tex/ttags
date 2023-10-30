use clap::Parser;
use std::collections::HashMap;

use tree_sitter_tags::TagsContext;
use tree_sitter_tags::TagsConfiguration;

use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;

const PATTERNS: [&str; 10] = [ "*.rs", "*.cpp", "*.hpp", "*.ipp", "*.cc", "*.hh", "*.c", "*.h", "*.py", "*.js" ];

#[derive(Debug)]
struct Entry {
    file: String,
    name: String,
    is_definition: bool,
    syntax_type_id: u32,
    row: usize,
    column: usize,
}

fn tokenize(file: &globwalk::DirEntry, conf: &TagsConfiguration) -> Vec<Entry> {
    let mut context = TagsContext::new();
    //println!("file: {}", file.path().display());
    let code = std::fs::read(file.path()).unwrap();
    let tags = context.generate_tags(
        &conf,
        &code,
        None,
    ).unwrap().0;

    let mut res: Vec<Entry> = Vec::new();
//println!("start tokenizing");
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
        //println!("token: {:?}", entry);
        res.push(entry);
    };

    return res;
}

// tags - all tags retrieved from one file
fn save_to_db(tags: Vec<Entry>, conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    if tags.len() == 0 {
        return Ok(());
    }
    println!("Storing {} items to database", tags.len());
    let tx = conn.transaction()?;
    //tx.execute("delete from tags where file = (?1)", [&tags.get(0).unwrap().file])?;
    for tag in tags {
        tx.execute("insert into tags (file, name, is_definition, syntax_type_id, row, column) values (?1, ?2, ?3, ?4, ?5, ?6)", [
            tag.file,
            tag.name,
            tag.is_definition.to_string(),
            tag.syntax_type_id.to_string(),
            tag.row.to_string(),
            tag.column.to_string()])?;
    }
    tx.commit()?;
    println!("Done!");
    Ok(())
}

fn scan(sw: Sender<globwalk::DirEntry>,
        rr: Receiver<Vec<Entry>>,
        conn: &mut rusqlite::Connection,
        path: &str)
            -> Result<(), globwalk::GlobError> {
    println!("scan begin");
    let walker = globwalk::GlobWalkerBuilder::from_patterns(
        path,
        &PATTERNS,
    )
    .follow_links(true)
    .case_insensitive(true)
    .build()?
    .into_iter()
    .filter_map(Result::ok);

    let mut count = 0;
    let mut res: Vec<Entry> = Vec::new();

    println!("scan end");

    // For each file found, ...
    for file in walker {
        if !file.file_type().is_file() {
            continue;
        }
        // Send the file to tokenizer.
        match sw.send(file) {
            Ok(_) => count += 1,
            Err(e) => println!("Error: {}", e)
        };
        //println!("Todo: {}", count);
        // Pickup results if there are any already.
        for tags in rr.try_iter() {
            count -= 1;
        //println!("remaining: {}", count);
            res.extend(tags);
        }
    }
    println!("Finished adding");

    // Pickup and wait for all the remaining results.
    for tags in rr.iter() {
        count -= 1;
        res.extend(tags);
        println!("remaining: {}", count);
        if count == 0 {
            break;
        };
    }
    println!("Picked up all results");

    match save_to_db(res, conn) {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("Error {}", e);
            return Ok(());
        }
    };
}

fn get_conf(confs : &mut HashMap<String, Rc<RefCell<TagsConfiguration>>>, ext : String) -> Option<Rc<RefCell<TagsConfiguration>>> {
    // Is there already existing configuration for given extension?
    match confs.get(&ext) {
        // Yes, there is some configuration...
        Some(conf) => {
            // Return it to caller...
            Some(conf.clone())
        },
        // There is no already existing configuration...
        _ => {
            let conf = match ext.as_str() {
                "rs" => (vec!["rs"], TagsConfiguration::new(
                    tree_sitter_rust::language(),
                    include_str!("tags_rust.scm"),
                    "")),
                "cc"|"hh"|"cpp"|"hpp"|"h"|"c" => (vec!["cc","hh","cpp","hpp"], TagsConfiguration::new(
                    tree_sitter_cpp::language(),
                    include_str!("tags_cpp.scm"),
                    "")),
//                "c"|"h" => (vec!["c","h"], TagsConfiguration::new(
//                    tree_sitter_c::language(),
//                    include_str!("tags_c.scm"),
//                    "")),
                "js" => (vec!["js"], TagsConfiguration::new(
                    tree_sitter_javascript::language(),
                    tree_sitter_javascript::TAGGING_QUERY,
                    tree_sitter_javascript::LOCALS_QUERY)),
                "py" => (vec!["py"], TagsConfiguration::new(
                    tree_sitter_python::language(),
                    tree_sitter_python::TAGGING_QUERY,
                    "")),
                _ => (vec![], Err(tree_sitter_tags::Error::InvalidLanguage)),
            };

            return match conf {
                (exts, Ok(conf)) => {
                    let val = Rc::new(RefCell::new(conf));
                    for ext in exts {
                        confs.insert(ext.to_string(), val.clone());
                    }
                    Option::Some(val)
                },
                _ => Option::None,
            };
        }
    }
}

// tokenizer needs a channel to receive commands to workers
// and to send results from workers
fn tokenizer(i: usize, rw: Receiver<globwalk::DirEntry>, sr: Sender<Vec<Entry>>) {
    //println!("tokenizer {}", i);

    let mut confs : HashMap<String, Rc<RefCell<TagsConfiguration>>> = HashMap::new();

    //println!("ready {}", i);
    for dir_entry in rw.iter() {
        //println!("Thread {} got a job! {}", i, dir_entry.path().display());
        let ext = match dir_entry.path().extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => String::from(ext),
            _ => continue,
        };
        //println!("It is {}", ext);
        match get_conf(&mut confs, ext) {
            Some(conf) => {
                let res = tokenize(&dir_entry, &*conf.borrow());
                match sr.send(res) {
                    Ok(_) => continue,
                    Err(_) => break,
                };
            },
            _ => {
            println!("Some error: {}", dir_entry.path().display());
                match sr.send(vec![]) {
                    Ok(_) => continue,
                    Err(_) => break,
                };
            }
        }
    }
}

fn prepare_db(conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
         PRAGMA synchronous = 0;
         PRAGMA cache_size = 1000000;
         PRAGMA locking_mode = EXCLUSIVE;
         PRAGMA temp_store = MEMORY;",
    )?;
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
    let (sw, rw) = bounded(num_cpus::get() * 10);
    // channel for reporting results from workers
    let (sr, rr) = unbounded();

    for i in 0..num_cpus::get() * 2 {
        let rw = rw.clone();
        let sr = sr.clone();
        thread::spawn(move || {
            tokenizer(i, rw, sr);
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
