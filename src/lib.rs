use easy_parallel::Parallel;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::rc::Rc;
use tree_sitter_tags::TagsConfiguration;
use tree_sitter_tags::TagsContext;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::{stdout, Write};

static GLOBAL_FILES_TOTAL_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_FILES_PROCESSED_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_FILES_SKIPPED_COUNT: AtomicUsize = AtomicUsize::new(0);

const PATTERNS: [&str; 10] = [ "*.rs", "*.cpp", "*.hpp", "*.ipp", "*.cc", "*.hh", "*.c", "*.h", "*.py", "*.js" ];

#[derive(Debug)]
struct TagEntry {
    file: String,
    name: String,
    is_definition: bool,
    syntax_type_id: u32,
    row: usize,
    column: usize,
}

fn new_tags_configuration(ext : &String) -> Option<TagsConfiguration> {
    match ext.as_str() {
        "text/x-rs" => Some(TagsConfiguration::new(
            tree_sitter_rust::language(),
            &fs::read_to_string(format!("{}/src/scm/tags_rust.scm", env!("CARGO_MANIFEST_DIR")))
                .expect("Can't read tags_rust.scm"),
            "").expect("Failed to create tags configuration")),
        "text/plain"|"text/x-c"|"text/x-c++" => Some(TagsConfiguration::new(
            tree_sitter_cpp::language(),
            &fs::read_to_string(format!("{}/src/scm/tags_cpp.scm", env!("CARGO_MANIFEST_DIR")))
                .expect("Can't read tags_cpp.scm"),
            "").expect("Failed to create tags configuration")),
        "text/x-js" => Some(TagsConfiguration::new(
            tree_sitter_javascript::language(),
            tree_sitter_javascript::TAGS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY).expect("Failed to create tags configuration")),
        "text/x-python" => Some(TagsConfiguration::new(
            tree_sitter_python::language(),
            tree_sitter_python::TAGS_QUERY,
            "").expect("Failed to create tags configuration")),
        _ => None,
    }
}

fn get_tags_configuration(confs : &mut HashMap<String, Rc<TagsConfiguration>>, ext : &String) -> Option<Rc<TagsConfiguration>> {
    match confs.get(ext) {
        Some(conf) => Some(conf.clone()),
        None => match new_tags_configuration(&ext) {
            Some(conf) => {
                let val = Rc::new(conf);
                confs.insert(ext.to_string(), val.clone());
                Some(val)
            },
            None => None,
        }
    }
}

fn tokenize(path: &std::path::Path, conf: &TagsConfiguration) -> Vec<TagEntry> {
    // Read source code to tokenize
    let code = std::fs::read(path)
        .map_err(|err| println!("Failed to read file ({}), error ({})", path.to_string_lossy(), err))
        .unwrap_or_default();
    // Create TreeSitter context and generate tags from the source code
    let mut context = TagsContext::new();
    let tags = context.generate_tags(&conf, &code, None).unwrap().0;

    let mut results: Vec<TagEntry> = Vec::new();
    for tag in tags {
        let tag: tree_sitter_tags::Tag = tag.unwrap();
        // println!("{}", conf.syntax_type_name(tag.syntax_type_id));
        let tag: TagEntry = TagEntry {
            file: path.to_string_lossy().to_string(),
            name: std::str::from_utf8(&code[tag.name_range]).unwrap_or("").to_string(),
            is_definition: tag.is_definition,
            syntax_type_id: tag.syntax_type_id,
            row: tag.span.start.row + 1,
            column: tag.span.start.column + 1,
        };
        results.push(tag);
    };
    results
}

fn save_tags_to_db(tags: &[TagEntry], conn: &mut rusqlite::Connection) -> Result<(), rusqlite::Error> {
    const CHUNKS: usize = 500;
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
    // Dropping indexes before pushing lot of data to a new, empty
    // table and creating the indexes after all insertions are done
    // is much faster.
    conn.execute("DROP INDEX IF EXISTS id", [])?;
    conn.execute("DROP INDEX IF EXISTS idx_tags", [])?;

    Ok(())
}

pub fn ttags_create(path: &str) {
    let walker = globwalk::GlobWalkerBuilder::from_patterns(path, &PATTERNS)
        .follow_links(true)
        .case_insensitive(true)
        .build()
        .expect("Failed to create a filesystem walker");

    let (tx_dir_entry, rx_dir_entry) = flume::unbounded::<globwalk::DirEntry>();
    let (tx_file_tokens, rx_file_tokens) = flume::unbounded::<Vec<TagEntry>>();

    walker.for_each(|result_with_dir_entry| {
        let dir_entry = result_with_dir_entry.unwrap();
        if dir_entry.file_type().is_file() {
            tx_dir_entry.send(dir_entry).unwrap();
            // Note that Relaxed ordering doesn't synchronize anything
            // except the global counter itself.
            let _ = GLOBAL_FILES_TOTAL_COUNT.fetch_add(1, Ordering::Relaxed);
        }
    });
    drop(tx_dir_entry);

    Parallel::new()
        .each(0..num_cpus::get(), |_| {
            let mut confs : HashMap<String, Rc<TagsConfiguration>> = HashMap::new();
            let mut buffer : Vec<TagEntry> = Vec::with_capacity(10000);

            let flags = magic::cookie::Flags::MIME_TYPE;
            let cookie = magic::Cookie::open(flags).expect("Failed to load magic");
            // let database = Default::default();
            let databases = [
                format!("{}/magic/c-lang", env!("CARGO_MANIFEST_DIR")),
            ].try_into().expect("Failed to load magic database");
            let cookie = cookie.load(&databases).expect("Failed to load magic database");

            for dir_entry in rx_dir_entry.iter() {
                let ext = cookie.file(dir_entry.path()).expect("Error");
                let conf = get_tags_configuration(&mut confs, &ext);
                match conf {
                    Some(conf) => {
                        // println!("Supported language ({}), ({})", dir_entry.path().to_string_lossy(), ext.as_str());
                        let file_tokens = tokenize(&dir_entry.path(), &conf);
                        buffer.extend(file_tokens);
                        if buffer.len() >= 10000 {
                            tx_file_tokens.send(buffer).unwrap();
                            buffer = Vec::with_capacity(10000);
                        }
                        // Note that Relaxed ordering doesn't synchronize anything
                        // except the global thread counter itself.
                        let _ = GLOBAL_FILES_PROCESSED_COUNT.fetch_add(1, Ordering::Relaxed);
                    },
                    None => {
                        println!("Unsupported language ({}), ({})", dir_entry.path().to_string_lossy(), ext.as_str());
                        // Note that Relaxed ordering doesn't synchronize anything
                        // except the global thread counter itself.
                        let _ = GLOBAL_FILES_SKIPPED_COUNT.fetch_add(1, Ordering::Relaxed);
                    }
                }
             }
            tx_file_tokens.send(buffer).unwrap();
            drop(tx_file_tokens);
        })
        .each(0..num_cpus::get(), |i| {
            let name = format!(".ttags.{}.db", i);
            let mut conn = rusqlite::Connection::open(name.clone()).expect(&format!("Error when opening database {}", name));
            prepare_db(&mut conn).expect(&format!("Error when preparing database {}", name));
            for file_tokens in rx_file_tokens.iter() {
                print!("PROCESSED {:?} + IGNORED {:?} / TOTAL {:?}\r", GLOBAL_FILES_PROCESSED_COUNT, GLOBAL_FILES_SKIPPED_COUNT, GLOBAL_FILES_TOTAL_COUNT);
                let _ = stdout().flush();
                //save_tags_to_db(&file_tokens, &mut conn)
                //    .expect(&format!("Error when inserting tags to database {}", name));
                drop(file_tokens)
            }
            conn.execute("CREATE INDEX IF NOT EXISTS id ON tags(id)", [])
                .expect(&format!("Error when preparing database {}", name));
            conn.execute("CREATE INDEX IF NOT EXISTS idx_tags ON tags(name, is_definition)", [])
                .expect(&format!("Error when preparing database {}", name));
        })
        .run();
}

#[cfg(test)]
mod lib_test;
