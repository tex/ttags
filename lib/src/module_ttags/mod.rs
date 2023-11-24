use std::collections::HashMap;

use tree_sitter_tags::TagsContext;
use tree_sitter_tags::TagsConfiguration;

use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use std::thread;
use std::rc::Rc;
use std::cell::RefCell;
use std::env;
use std::fs;

#[cfg(test)]
const DEF: bool = true;
#[cfg(test)]
const REF: bool = false;

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

fn tokenize(path: &std::path::Path, conf: &TagsConfiguration) -> Vec<Entry> {
    let mut context = TagsContext::new();
    //println!("file: {}", file.path().display());
    let code = std::fs::read(path).map_err(|e| format!("{}: {}", path.to_string_lossy(), e)).unwrap();
    let tags = context.generate_tags(&conf, &code, None).unwrap().0;

    let mut res: Vec<Entry> = Vec::new();
//println!("start tokenizing");
    for tag in tags {
        let tag: tree_sitter_tags::Tag = tag.unwrap();
        let entry: Entry = Entry {
            file: path.to_string_lossy().to_string(),
            // file: path.to_str().unwrap_or("").to_string(),
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

fn scan(sw: Sender<globwalk::DirEntry>,
        rr: Receiver<Vec<Entry>>,
        path: &str)
            -> Result<Vec<Entry>, globwalk::GlobError> {
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

    return Ok(res);
}

fn get_conf(confs : &mut HashMap<String, Rc<RefCell<TagsConfiguration>>>, ext : String) -> Rc<RefCell<TagsConfiguration>> {
    // Is there already existing configuration for given extension?
    match confs.get(&ext) {
        // Yes, there is some configuration...
        Some(conf) => {
            // Return it to caller...
            conf.clone()
        },
        // There is no already existing configuration...
        _ => {
            println!(" creating new one");
            let conf = match ext.as_str() {
                "rs" => (vec!["rs"], TagsConfiguration::new(
                    tree_sitter_rust::language(),
                    &fs::read_to_string(format!("{}/src/module_ttags/tags_rust.scm", env!("CARGO_MANIFEST_DIR")))
                        .expect("Can't read tags_rust.scm"),
                    // include_str!("tags_rust.scm"),
                    "")),
                // Oh yes, lots of c++ source code can and is in c file...
                "cc"|"hh"|"cpp"|"hpp"|"ipp"|"h"|"c" => (vec!["cc","hh","cpp","hpp","ipp", "h","c"], TagsConfiguration::new(
                    tree_sitter_cpp::language(),
                    &fs::read_to_string(format!("{}/src/module_ttags/tags_cpp.scm", env!("CARGO_MANIFEST_DIR")))
                        .expect("Can't read tags_cpp.scm"),
                    // include_str!("tags_cpp.scm"),
                    "")),
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
                    val
                },
                (exts, Result::Err(e)) => {
                    println!("Problem in scm of {:?}: {}", exts, e);
                    panic!();
                },
            };
        }
    }
}

// tokenizer needs a channel to receive commands to workers
// and to send results from workers
fn tokenizer(rw: Receiver<globwalk::DirEntry>, sr: Sender<Vec<Entry>>) {
    //println!("tokenizer {}", i);

    let mut confs : HashMap<String, Rc<RefCell<TagsConfiguration>>> = HashMap::new();

    //println!("ready {}", i);
    for dir_entry in rw.iter() {
        //println!("Thread {} got a job! {}", i, dir_entry.path().display());
        let ext = match dir_entry.path().extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => String::from(ext),
            _ => continue,
        };
        let conf = get_conf(&mut confs, ext);
        let res = tokenize(&dir_entry.path(), &*conf.borrow());
        match sr.send(res) {
            Ok(_) => continue,
            Err(_) => break,
        };
    }
}

#[test]
fn test_tokenize_cpp() {
    let mut confs : HashMap<String, Rc<RefCell<TagsConfiguration>>> = HashMap::new();
    let conf = get_conf(&mut confs, "cpp".to_string());
    let res = tokenize(std::path::Path::new(&format!("{}/src/module_ttags/test.cpp", env!("CARGO_MANIFEST_DIR"))), &*conf.borrow());
println!("{:?}", res);
    let q = |res : &Vec<Entry>, name, row, is_definition| {
        res.iter().any(|entry| entry.name == name
            && entry.is_definition == is_definition
            && entry.row == row) };
    let qq = |res : &Vec<Entry>, name, row, is_definition| {
        assert!(q(&res, name, row, is_definition), "{}, row: {}, {}, not found in {:?}", name, row, is_definition, res) };
    let nq = |res : &Vec<Entry>, name, row, is_definition| {
        assert!(!q(&res, name, row, is_definition), "{}, row: {}, {}, found in {:?}", name, row, is_definition, res) };

    qq(&res, "Class_1", 1, DEF); // class
    qq(&res, "Class_1", 4, DEF); // constructor
    nq(&res, "m_Variable_1", 4, REF);
    nq(&res, "m_Struct_1", 4, REF);
    qq(&res, "Setup", 5, DEF);
    nq(&res, "m_Variable_1", 5, REF);
    qq(&res, "Work", 6, DEF);
    qq(&res, "m_Variable_1", 8, DEF);
    qq(&res, "m_Struct_1", 9, DEF);
    qq(&res, "Work", 12, DEF);
    // Local variables not wanted!
    nq(&res, "local_1", 14, DEF);

    qq(&res, "m_Variable_1", 15, REF);

    // Don't want this in database as there is
    // many many of such in any source code.
    // Use grep to find them.
    nq(&res, "m_Struct_1", 16, REF);
    qq(&res, "Test", 16, REF);

    qq(&res, "MyStruct", 19, REF);
    nq(&res, "myStruct", 19, DEF);

    nq(&res, "myStruct", 20, REF);
    qq(&res, "Todo", 20, REF);
}

// tags - all tags retrieved from one file
fn save_to_db(tags: Vec<Entry>, conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    println!("Storing {} items to database", tags.len());
    let tx = conn.transaction()?;
    let chunks = 50;
    let mut st = format!("insert into tags values{}", " (NULL, ?, ?, ?, ?, ?, ?),".repeat(chunks));
    st.pop(); // pop the last character: ','
    println!("{}", st);
    {
    let mut stm = tx.prepare_cached(st.as_str()).unwrap();
    //tx.execute("delete from tags where file = (?1)", [&tags.get(0).unwrap().file])?;
    for tag_chunks in tags.chunks(chunks) {
        let mut param_values: Vec<_> = Vec::new();
        for tag in tag_chunks {
            param_values.push(&tag.file as &dyn rusqlite::ToSql);
            param_values.push(&tag.name as &dyn rusqlite::ToSql);
            param_values.push(&tag.is_definition as &dyn rusqlite::ToSql);
            param_values.push(&tag.syntax_type_id as &dyn rusqlite::ToSql);
            param_values.push(&tag.row as &dyn rusqlite::ToSql);
            param_values.push(&tag.column as &dyn rusqlite::ToSql);
        }
            if tag_chunks.len() != chunks {
                st = format!("insert into tags values{}", " (NULL, ?, ?, ?, ?, ?, ?),".repeat(tag_chunks.len()));
                st.pop(); // pop the last character: ','
                stm = tx.prepare_cached(st.as_str()).unwrap();
            }
            stm.execute(&*param_values).unwrap();
        }
    }
    tx.commit()?;
    conn.execute("create index if not exists id on tags(id)", [])?;
    conn.execute("create index if not exists idx_tags on tags(name, is_definition)", [])?;
    Ok(())
}

fn prepare_db(conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    conn.execute_batch(
        "PRAGMA journal_mode = OFF;
         PRAGMA synchronous = 0;
         PRAGMA cache_size = 1000000;
         PRAGMA locking_mode = EXCLUSIVE;
         PRAGMA temp_store = MEMORY;",
    )?;
    conn.execute("drop table if exists tags", [])?;
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
    conn.execute("drop index if exists id", [])?;
    conn.execute("drop index if exists idx_tags", [])?;
    Ok(())
}

pub fn ttags_create(conn: &mut rusqlite::Connection, path: &str) -> Result<(), globwalk::GlobError> {
    // channel for giving commands to workers
    let (sw, rw) = bounded(num_cpus::get() * 10);
    // channel for reporting results from workers
    let (sr, rr) = unbounded();

    for i in 0..num_cpus::get() * 2 {
        let rw = rw.clone();
        let sr = sr.clone();
        thread::spawn(move || {
            tokenizer(rw, sr);
        });
    };

    // scanner needs to be able to give commands to workers (sw)
    // and to retrieve results from workers (rr)
    let res = scan(sw, rr, path);

    match save_to_db(res?, conn) {
        Ok(_) => return Ok(()),
        Err(e) => {
            println!("Error {}", e);
            return Ok(());
        }
    };
}

