use std::collections::HashMap;

use tree_sitter_tags::TagsContext;
use tree_sitter_tags::TagsConfiguration;

extern crate rayon;
use rayon::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::env;
use std::fs;
use std::cmp;

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

#[test]
fn test_tokenize_cpp() {
    let mut confs : HashMap<String, Rc<RefCell<TagsConfiguration>>> = HashMap::new();
    let conf = get_tags_configuration(&mut confs, "cpp".to_string());

    let res = tokenize(
        std::path::Path::new(
            &format!("{}/src/module_ttags/test.cpp", env!("CARGO_MANIFEST_DIR"))),
        &*conf.borrow());

    let q = |res : &Vec<Entry>, name, row, is_definition| {
        res.iter().any(|entry|
            entry.name == name &&
            entry.is_definition == is_definition &&
            entry.row == row) };

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

fn get_tags_configuration(confs : &mut HashMap<String, Rc<RefCell<TagsConfiguration>>>, ext : String) -> Rc<RefCell<TagsConfiguration>> {
    // Is there already existing configuration for given extension?
    match confs.get(&ext) {
        // Yes, there is some configuration...
        Some(conf) => {
            // Return it to caller...
            conf.clone()
        },
        // There is no already existing configuration...
        _ => {
            let conf = match ext.as_str() {
                "rs" =>
                    (vec!["rs"], TagsConfiguration::new(
                        tree_sitter_rust::language(),
                        &fs::read_to_string(format!("{}/src/module_ttags/tags_rust.scm", env!("CARGO_MANIFEST_DIR")))
                            .expect("Can't read tags_rust.scm"),
                        "")),
                // Oh yes, lots of c++ source code is in c file...
                "cc"|"hh"|"cpp"|"hpp"|"ipp"|"h"|"c" =>
                    (vec!["cc","hh","cpp","hpp","ipp", "h","c"], TagsConfiguration::new(
                        tree_sitter_cpp::language(),
                        &fs::read_to_string(format!("{}/src/module_ttags/tags_cpp.scm", env!("CARGO_MANIFEST_DIR")))
                            .expect("Can't read tags_cpp.scm"),
                        "")),
                "js" =>
                    (vec!["js"],
                        TagsConfiguration::new(
                        tree_sitter_javascript::language(),
                        tree_sitter_javascript::TAGGING_QUERY,
                        tree_sitter_javascript::LOCALS_QUERY)),
                "py" =>
                    (vec!["py"], TagsConfiguration::new(
                        tree_sitter_python::language(),
                        tree_sitter_python::TAGGING_QUERY,
                        "")),
                _ =>
                    (vec![], Err(tree_sitter_tags::Error::InvalidLanguage)),
            };

            return match conf {
                (exts, Ok(conf)) => {
                    let val = Rc::new(RefCell::new(conf));
                    for ext in exts {
                        confs.insert(ext.to_string(), val.clone());
                    }
                    val
                },
                (exts, Err(e)) => {
                    println!("Problem in scm of {:?}: {}", exts, e);
                    panic!();
                },
            };
        }
    }
}

fn tokenize(path: &std::path::Path, conf: &TagsConfiguration) -> Vec<Entry> {
    // Read source code to tokenize
    let code = std::fs::read(path)
        .map_err(|err| println!("Failed to read file ({}), error ({})", path.to_string_lossy(), err))
        .unwrap_or_default();
    // Create TreeSitter context and generate tags from the source code
    let mut context = TagsContext::new();
    let tags = context.generate_tags(&conf, &code, None).unwrap().0;

    let mut results: Vec<Entry> = Vec::new();
    for tag in tags {
        let tag: tree_sitter_tags::Tag = tag.unwrap();
        let entry: Entry = Entry {
            file: path.to_string_lossy().to_string(),
            name: std::str::from_utf8(&code[tag.name_range]).unwrap_or("").to_string(),
            is_definition: tag.is_definition,
            syntax_type_id: tag.syntax_type_id,
            row: tag.span.start.row + 1,
            column: tag.span.start.column + 1,
        };
        results.push(entry);
    };
    results
}

fn tokenize_chunk(dir_entries: &[globwalk::DirEntry]) -> Vec<Entry> {
    let mut confs : HashMap<String, Rc<RefCell<TagsConfiguration>>> = HashMap::new();
    let mut results : Vec<Entry> = Vec::new();

    for dir_entry in dir_entries {
        let ext = dir_entry
            .path()
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .expect(&format!("Failed to get extension of file ({})", dir_entry.path().to_string_lossy()));
        let conf = get_tags_configuration(&mut confs, ext.to_string());
        results.extend(tokenize(&dir_entry.path(), &*conf.borrow()));
    }
    results
}

fn save_chunk_to_db(index: usize, tags: &[Entry]) {
    let name = format!(".ttags.{}.db", index);
    let mut conn = rusqlite::Connection::open(name.clone()).expect(&format!("Error when opening database {}", name));
    prepare_db(&mut conn).expect(&format!("Error when preparing database {}", name));
    save_tags_to_db(tags, &mut conn).expect(&format!("Error when inserting tags to database {}", name));
}

fn save_tags_to_db(tags: &[Entry], conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    // Dropping indexes before pushing lot of data to a new, empty
    // table and creating the indexes after all insertions are done
    // is supposedly faster.
    conn.execute("DROP INDEX IF EXISTS id", [])?;
    conn.execute("DROP INDEX IF EXISTS idx_tags", [])?;

    const CHUNKS: usize = 50;
    let mut st = format!("INSERT INTO tags VALUES{}", " (NULL, ?, ?, ?, ?, ?, ?),".repeat(CHUNKS));
    st.pop(); // pop the last character: ','

    let mut stm = conn.prepare_cached(st.as_str()).unwrap();
    for tag_chunks in tags.chunks(CHUNKS) {
        let mut param_values: Vec<_> = Vec::new();
        for tag in tag_chunks {
            param_values.push(&tag.file as &dyn rusqlite::ToSql);
            param_values.push(&tag.name as &dyn rusqlite::ToSql);
            param_values.push(&tag.is_definition as &dyn rusqlite::ToSql);
            param_values.push(&tag.syntax_type_id as &dyn rusqlite::ToSql);
            param_values.push(&tag.row as &dyn rusqlite::ToSql);
            param_values.push(&tag.column as &dyn rusqlite::ToSql);
        }
        if tag_chunks.len() != CHUNKS {
            st = format!("INSERT INTO tags VALUES{}", " (NULL, ?, ?, ?, ?, ?, ?),".repeat(tag_chunks.len()));
            st.pop(); // pop the last character: ','
            stm = conn.prepare_cached(st.as_str()).unwrap();
        }
        stm.execute(&*param_values).unwrap();
    }

    conn.execute("CREATE INDEX IF NOT EXISTS id ON tags(id)", [])?;
    conn.execute("CREATE INDEX IF NOT EXISTS idx_tags ON tags(name, is_definition)", [])?;

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
    conn.execute("DROP TABLE IF EXISTS tags", [])?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY,
            file TEXT NOT NULL,
            name TEXT NOT NULL,
            is_definition BOOL,
            syntax_type_id INTEGER,
            row INTEGER,
            column INTEGER
         )",
        [],
    )?;

    Ok(())
}

#[test]
fn compute_chunk_size_test()
{
    // O [10, 10, 10, 1]
    // O [9, 9, 9, 4]
    // O [8, 8, 8, 7]
    // X [7, 7, 7, 10]

    assert_eq!(compute_chunk_size(31, 4), 8);
}

// Chunks size as big as long as the remaining last chunk
// is as big as possible but not bigger than previous chunks size.
// Simple division 31/4 => 7 => 7 + 7 + 7 + 7 + 3
// This function chooses 8 + 8 + 8 + 7
fn compute_chunk_size(size: usize, chunks: usize) -> usize {
    let mut chunk_size = size / (chunks - 1);
    let mut last_chunk_size = size % (chunks - 1);
    while last_chunk_size + 1 * (chunks - 1) < chunk_size {
        last_chunk_size += 1 * (chunks - 1);
        chunk_size -= 1;
    }
    chunk_size
}

pub fn ttags_create(path: &str) {
    let walker = globwalk::GlobWalkerBuilder::from_patterns(path, &PATTERNS)
        .follow_links(true)
        .case_insensitive(true)
        .build().expect("Failed to create a filesystem walker");

    let files: Vec<_> = walker
        .filter_map(|result_with_dir_entry| {
            result_with_dir_entry
                .ok()
                .and_then(|dir_entry| {
                    if dir_entry.file_type().is_file() {
                        Some(dir_entry)
                    } else {
                        None
                    }
                })
        })
        .collect();

    let results: Vec<_> = files
        .par_chunks(cmp::max(1, num_cpus::get() - 1))
        .flat_map(|chunk| tokenize_chunk(chunk))
        .collect();

    // According to https://www.sqlite.org/limits.html the default
    // maximum number of attached databases in sqlite is 10.
    results
        .par_chunks(compute_chunk_size(
            results.len(), cmp::min(10, cmp::max(1, num_cpus::get() - 1))))
        .enumerate()
        .for_each(|(index, chunk)| save_chunk_to_db(index, chunk) );
}

