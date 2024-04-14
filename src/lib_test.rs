use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;
use tree_sitter_tags::TagsConfiguration;

use super::Entry;
use super::get_tags_configuration;
use super::tokenize;

#[cfg(test)]
const DEF: bool = true;
#[cfg(test)]
const REF: bool = false;

#[test]
fn test_tokenize_cpp() {
    let mut confs : HashMap<String, Rc<RefCell<TagsConfiguration>>> = HashMap::new();
    let conf = get_tags_configuration(&mut confs, "cpp".to_string());

    let res = tokenize(
        std::path::Path::new(
            &format!("{}/tests/test.cpp", env!("CARGO_MANIFEST_DIR"))),
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
    // <del>Local variables not wanted!</del>
    // Oh, this is needed as it catches other required
    // tokens.
    qq(&res, "local_1", 14, DEF);

    qq(&res, "m_Variable_1", 15, REF);

    // Don't want this in database as there is
    // many many of such in any source code.
    // Use grep to find them.
    nq(&res, "m_Struct_1", 16, REF);
    qq(&res, "Test", 16, REF);

    qq(&res, "MyStruct", 19, REF);
    // nq(&res, "myStruct", 19, DEF);

    nq(&res, "myStruct", 20, REF);
    qq(&res, "Todo", 20, REF);

    qq(&res, "GLOBAL_ARRAY", 23, DEF);
    qq(&res, "GLOBAL_ARRAY", 26, REF);
    qq(&res, "GLOBAL_ARRAY", 27, REF);

    qq(&res, "TestObject", 30, REF);
    qq(&res, "Int", 30, REF);
    qq(&res, "TEST_OBJECT_ID", 30, REF);

    qq(&res, "SOMETHING", 33, REF);
    qq(&res, "uint32_t", 33, REF);
    qq(&res, "ARRAY", 33, REF);
    qq(&res, "Something", 33, REF);

    qq(&res, "specificItem", 34, REF);

    qq(&res, "platformID", 37, DEF);
    qq(&res, "ClassAttributes", 38, DEF);
    qq(&res, "InstanceAttributes", 39, DEF);

    qq(&res, "g_Global_1", 42, REF);
    qq(&res, "g_Global_2", 42, REF);
    qq(&res, "someFunction", 42, REF);
    qq(&res, "CONST_1", 43, REF);
    qq(&res, "CONST_2", 45, REF);

    qq(&res, "CONST", 49, REF);
}
