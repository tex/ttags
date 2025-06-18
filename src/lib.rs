use easy_parallel::Parallel;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;
use std::sync::Arc;
use std::io;
use std::io::{stdout, Seek, SeekFrom, Read, Write, BufReader, BufWriter};
use std::time::UNIX_EPOCH;
use serde::{Serialize, Deserialize};
use std::fs::File;
use itertools::Itertools;
use regex::Regex;
use std::fs::OpenOptions;
use std::io::BufRead;

static GLOBAL_FILES_TOTAL_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_FILES_PROCESSED_COUNT: AtomicUsize = AtomicUsize::new(0);
static GLOBAL_FILES_SKIPPED_COUNT: AtomicUsize = AtomicUsize::new(0);

const PATTERNS: [&str; 10] = [ "*.rs", "*.cpp", "*.hpp", "*.ipp", "*.cc", "*.hh", "*.c", "*.h", "*.py", "*.js" ];

fn new_tags_configuration(ext : &String) -> Option<tree_sitter_tags::TagsConfiguration> {
    match ext.as_str() {
        "text/x-rs" => Some(tree_sitter_tags::TagsConfiguration::new(
            tree_sitter_rust::LANGUAGE.into(),
            &fs::read_to_string(format!("{}/src/scm/tags_rust.scm", env!("CARGO_MANIFEST_DIR")))
                .expect("Can't read tags_rust.scm"),
            "").expect("Failed to create tags configuration")),
        "text/plain"|"text/x-c"|"text/x-c++" => Some(tree_sitter_tags::TagsConfiguration::new(
            tree_sitter_cpp::LANGUAGE.into(),
            &fs::read_to_string(format!("{}/src/scm/tags_cpp.scm", env!("CARGO_MANIFEST_DIR")))
                .expect("Can't read tags_cpp.scm"),
            "").expect("Failed to create tags configuration")),
        "text/x-js" => Some(tree_sitter_tags::TagsConfiguration::new(
            tree_sitter_javascript::LANGUAGE.into(),
            tree_sitter_javascript::TAGS_QUERY,
            tree_sitter_javascript::LOCALS_QUERY).expect("Failed to create tags configuration")),
        "text/x-python" => Some(tree_sitter_tags::TagsConfiguration::new(
            tree_sitter_python::LANGUAGE.into(),
            tree_sitter_python::TAGS_QUERY,
            "").expect("Failed to create tags configuration")),
        _ => None,
    }
}

fn get_tags_configuration(
    confs : &mut HashMap<String, Rc<tree_sitter_tags::TagsConfiguration>>,
    ext : &String)
-> Option<Rc<tree_sitter_tags::TagsConfiguration>>
{
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

#[derive(Serialize, Deserialize, Debug)]
struct TagEntry {
    file_index: usize,
    is_definition: bool,
    syntax_type_id: u32,
    row: usize,
    column: usize,
}

impl TagEntry {
    fn new() -> Self {
        TagEntry {
file_index: 0,
is_definition: false,
syntax_type_id: 0,
row: 0,
column: 0,
    }
    }
}

// This is separate because there is much less
// number of files then extracted tags.
#[derive(Serialize, Deserialize, Debug)]
struct TagFile {
    path: String,
    modification: u64,
}

struct TagData {
    name: HashMap<String, usize>,
    name_entry: Vec<Vec<usize>>,
    file: Vec<TagFile>,
    entry: Vec<TagEntry>,
}

impl TagData {
    fn new() -> Self {
        TagData {
            name: HashMap::new(),
            name_entry: vec![],
            file: vec![],
            entry: vec![],
        }
    }
}

fn read_regex_patterns(filename: &str) -> Vec<(Regex, String)> {
    let lines = std::fs::read(filename)
        .map_err(|err| println!("Failed to read file ({}), error ({})", filename, err))
        .unwrap_or_default();

    let mut regexes = Vec::new();

    for line in lines.lines() {
        let line = line.expect("Failed");
        let parts: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            let re = Regex::new(&parts[0].to_string()).unwrap();
            regexes.push((re, parts[1].to_string()));
        }
    }
    regexes
}

fn apply_regex_replacements(regexes: &Vec<(Regex, String)>, text: &str) -> String {
    let mut result = text.to_string();
    for (re, replacement) in regexes {
        result = re.replace_all(&result, replacement.as_str()).to_string();
    }
    result
}

fn tokenize(file: &String, file_index: usize, conf: &tree_sitter_tags::TagsConfiguration, tag_data: &Mutex<TagData>, regexes: &Vec<(Regex, String)>) {
    // Read source code to tokenize
    let lines = std::fs::read(file)
        .map_err(|err| println!("Failed to read file ({}), error ({})", file, err))
        .unwrap_or_default();

    let code = apply_regex_replacements(regexes, &String::from_utf8_lossy(&lines)).into_bytes();

    // Create TreeSitter context and generate tags from the source code
    let mut context = tree_sitter_tags::TagsContext::new();
    let tags = context.generate_tags(&conf, &code, None).unwrap().0;

    let mut tag_data = tag_data.lock().unwrap();

    for tag in tags {
        let tag: tree_sitter_tags::Tag = tag.unwrap();

        let is_definition_to_string = |i:bool| -> &str { if i { "definition" } else { "reference" } };
        let name = std::str::from_utf8(&code[tag.name_range.clone()]).unwrap_or("").to_string();
        println!("file: {:?}, docs: {:?}, span: {:?}, type: {}.{}, name: {:?}", file, tag.docs, tag.span, is_definition_to_string(tag.is_definition), &conf.syntax_type_name(tag.syntax_type_id), name);

        tag_data.entry.push(
            TagEntry {
                file_index: file_index,
                is_definition: tag.is_definition,
                syntax_type_id: tag.syntax_type_id,
                row: tag.span.start.row + 1,
                column: tag.span.start.column + 1,
            });
        let entry_index = tag_data.entry.len() - 1;

        if !tag_data.name.contains_key(&name) {
            tag_data.name_entry.push(vec![entry_index]);
            let name_entry_index = tag_data.name_entry.len() - 1;
            tag_data.name.insert(name, name_entry_index);
        } else {
            let name_entry_index = *tag_data.name.get(&name).unwrap();
            tag_data.name_entry.get_mut(name_entry_index).unwrap().push(entry_index);
        }
    };
}

fn tokenize_thread(rx_dir_entry: crossbeam_channel::Receiver<(String, usize)>, tag_data: &Mutex<TagData>) {
    let mut confs : HashMap<String, Rc<tree_sitter_tags::TagsConfiguration>> = HashMap::new();

    let flags = magic::cookie::Flags::MIME_TYPE;
    let cookie = magic::Cookie::open(flags).expect("Failed to load magic");
    let databases = [
        format!("{}/magic/c-lang", env!("CARGO_MANIFEST_DIR")),
    ].try_into().expect("Failed to load magic database");
    let cookie = cookie.load(&databases).expect("Failed to load magic database");

    let regexes = read_regex_patterns("ttags.regex");

    for (file, file_index) in rx_dir_entry.iter() {
        let ext = cookie.file(&file).expect("Error");
        let conf = get_tags_configuration(&mut confs, &ext);
        match conf {
            Some(conf) => {
                tokenize(&file, file_index, &conf, &tag_data, &regexes);
                let _ = GLOBAL_FILES_PROCESSED_COUNT.fetch_add(1, Ordering::Relaxed);
                if GLOBAL_FILES_PROCESSED_COUNT.load(Ordering::Relaxed) % 100 == 0 {
                    print!("PROCESSED {:?} + IGNORED {:?} / TOTAL {:?}\r",
                        GLOBAL_FILES_PROCESSED_COUNT, GLOBAL_FILES_SKIPPED_COUNT, GLOBAL_FILES_TOTAL_COUNT);
                    let _ = stdout().flush();
                }
            },
            None => {
                println!("Unsupported language ({}), ({})", file, ext.as_str());
                let _ = GLOBAL_FILES_SKIPPED_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        }
    }
}

fn find_files(tx_dir_entry : crossbeam_channel::Sender<(String, usize)>, tag_data: &Mutex<TagData>) {
    let mut tag_data = tag_data.lock().unwrap();
    globwalk::GlobWalkerBuilder::from_patterns(".", &PATTERNS)
        .follow_links(true)
        .case_insensitive(true)
        .build()
        .expect("Failed to create a filesystem walker")
        .for_each(|result_with_dir_entry| {
            let dir_entry = result_with_dir_entry.unwrap();
            if dir_entry.file_type().is_file() {
                //file_modified_time_in_seconds

                let path = dir_entry.path().to_string_lossy().to_string();
                let modification = fs::metadata(&path).unwrap()
                    .modified().unwrap()
                    .duration_since(UNIX_EPOCH).unwrap()
                    .as_secs();

                let tag_file = TagFile {
                    path: path.clone(),
                    modification: modification,
                };
                tag_data.file.push(tag_file);
                let file_index = tag_data.file.len() - 1;

                tx_dir_entry.send((path, file_index)).unwrap();
                let _ = GLOBAL_FILES_TOTAL_COUNT.fetch_add(1, Ordering::Relaxed);
            }
        });
    drop(tx_dir_entry);
}

pub fn ttags_create(path: &str) {

    std::env::set_current_dir(&path).expect("Failed to change working directory");

    let tag_data = Arc::new(Mutex::new(TagData::new()));

    let (tx_dir_entry, rx_dir_entry) = crossbeam_channel::unbounded();

    Parallel::new()
        .add(|| {
            find_files(tx_dir_entry, &*tag_data.clone());
        })
        .each(0..num_cpus::get(), |_| {
            tokenize_thread(rx_dir_entry, &*tag_data.clone());
        })
        .run();

    let tag_data = tag_data.lock().unwrap();

    let mut name: Vec<(String, usize)> = Vec::new();
    for (k, v) in tag_data.name.iter().sorted_by_key(|x| x.0) {
        name.push((k.clone(), *v));
    }
    let mut buf_writer = BufWriter::new(OpenOptions::new().write(true).create(true)
            .open(".ttags.name.bin").expect("Failed to create file"));
    bincode::serialize_into(&mut buf_writer, &name).expect("Serialization failed");
    bincode::serialize_into(
        BufWriter::new(OpenOptions::new().write(true).create(true)
            .open(".ttags.name_entry.bin").expect("Failed to create file")),
        &tag_data.name_entry).expect("Serialization failed");
    bincode::serialize_into(
        BufWriter::new(OpenOptions::new().write(true).create(true)
            .open(".ttags.file.bin").expect("Failed to create file")),
        &tag_data.file).expect("Serialization failed");
    {
        let mut buf_writer = BufWriter::new(OpenOptions::new().write(true).create(true)
            .open(".ttags.entry.bin").expect("Failed to create file"));
        for entry in tag_data.entry.iter() {
            bincode::serialize_into(&mut buf_writer, &entry).expect("Serialization failed");
        }
    }
}

pub fn ttags_complete(symbol: &str) {
    let name: Vec<(String, usize)> = bincode::deserialize_from(
        BufReader::new(File::open(".ttags.name.bin").expect("File not found")))
        .expect("Deserialization failed");
    // Step 1: Binary search to find the first potential match
    let mut i = match name.binary_search_by(|t| {
        // Compare the item value with the regex pattern
        if *symbol == *t.0 {
            std::cmp::Ordering::Equal
        } else if *t.0 < *symbol {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }) {
        Ok(idx) => idx, // Exact match found
        Err(idx) => idx, // No exact match, but this is where matches could start
    };
    let mut c = 0;
    while i < name.len() && c < 10 {

let name_entry_index = name.get(i).unwrap().1;

        let file: Vec<TagFile> = bincode::deserialize_from(
            BufReader::new(File::open(".ttags.file.bin").expect("File not found")))
            .expect("Deserialization failed");
        let name_entry: Vec<Vec<usize>> = bincode::deserialize_from(
            BufReader::new(File::open(".ttags.name_entry.bin").expect("File not found")))
            .expect("Deserialization failed");


    let mut entry_file = File::open(".ttags.entry.bin").expect("File not found");
    let entry_size: usize = bincode::serialized_size(&TagEntry::new()).unwrap() as usize;

        for entry_index in &name_entry[name_entry_index] {
    let offset = entry_size * entry_index;
    entry_file.seek(SeekFrom::Start(offset as u64)).expect("File seek failed");

    // Read the bytes for the struct
    let mut buffer = vec![0u8; entry_size];
    entry_file.read_exact(&mut buffer).expect("File read failed");

    let mut confs : HashMap<String, Rc<tree_sitter_tags::TagsConfiguration>> = HashMap::new();
    let flags = magic::cookie::Flags::MIME_TYPE;
    let cookie = magic::Cookie::open(flags).expect("Failed to load magic");
    let databases = [
        format!("{}/magic/c-lang", env!("CARGO_MANIFEST_DIR")),
    ].try_into().expect("Failed to load magic database");
    let cookie = cookie.load(&databases).expect("Failed to load magic database");


    // Deserialize the bytes into the struct
    let entry: TagEntry = bincode::deserialize(&buffer).unwrap();

                let file: &TagFile = &file[entry.file_index];


        let ext = cookie.file(&file.path).expect("Error");
        let conf = get_tags_configuration(&mut confs, &ext);
        match conf {
            Some(conf) => {
                println!("{}:{}:{}:{}:{}",
                    file.path, entry.row, name.get(i).unwrap().0, entry.is_definition, &conf.syntax_type_name(entry.syntax_type_id) /* should be line*/);
            },
            None => {
                println!("{}:{}:{}:{}:{}",
                    file.path, entry.row, name.get(i).unwrap().0, entry.is_definition, "?!?!?!" /* should be line*/);
                }
        }

        i += 1;
            c += 1;
}
}
}

pub fn ttags_find(is_definition: bool, pattern: &str) {
    // let re = Regex::new(pattern).unwrap();

    let name: Vec<(String, usize)> = bincode::deserialize_from(
        BufReader::new(File::open(".ttags.name.bin").expect("File not found")))
        .expect("Deserialization failed");
    // Step 1: Binary search to find the first potential match
    let i = match name.binary_search_by(|t| {
        // Compare the item value with the regex pattern
        if *pattern == *t.0 {
            std::cmp::Ordering::Equal
        } else if *t.0 < *pattern {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    }) {
        Ok(idx) => idx, // Exact match found
        Err(idx) => idx, // No exact match, but this is where matches could start
    };
let name_entry_index = name.get(i).unwrap().1;

        let file: Vec<TagFile> = bincode::deserialize_from(
            BufReader::new(File::open(".ttags.file.bin").expect("File not found")))
            .expect("Deserialization failed");
        let name_entry: Vec<Vec<usize>> = bincode::deserialize_from(
            BufReader::new(File::open(".ttags.name_entry.bin").expect("File not found")))
            .expect("Deserialization failed");


    let mut entry_file = File::open(".ttags.entry.bin").expect("File not found");
    let entry_size: usize = bincode::serialized_size(&TagEntry::new()).unwrap() as usize;

        for entry_index in &name_entry[name_entry_index] {
    let offset = entry_size * entry_index;
    entry_file.seek(SeekFrom::Start(offset as u64)).expect("File seek failed");

    // Read the bytes for the struct
    let mut buffer = vec![0u8; entry_size];
    entry_file.read_exact(&mut buffer).expect("File read failed");

    // Deserialize the bytes into the struct
    let entry: TagEntry = bincode::deserialize(&buffer).unwrap();

    let get_context = |path, line| -> String {
            let file = File::open(path).expect("File not found!");
            let reader = io::BufReader::new(file);
            if let Some(Ok(line)) = reader.lines().nth(line-1) {
                line.trim_start().to_owned()
            } else { String::from("") }
        };


            if entry.is_definition == is_definition {
                let file: &TagFile = &file[entry.file_index];
                println!("{}:{}:{}",
                    file.path, entry.row, get_context(&file.path, entry.row));
            }
        }



//    let mut results = Vec::new();
//
//    // Step 2: Linear search from the start index to gather matches
//    for item in &data[start_index..] {
//        if re.is_match(&item.value) {
//            results.push(item);
//        } else if item.value > pattern {
//            break; // Stop if we've moved past the range of possible matches
//        }
// 
}

#[cfg(test)]
mod lib_test;
