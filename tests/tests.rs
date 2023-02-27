use path_trav::*;
use std::{io::ErrorKind, path};

#[test]
fn true_path() {
    let base = path::Path::new("./");
    let rel = path::Path::new("/etc/passwd");

    assert_eq!(Ok(true), base.is_path_trav(&rel))
}

#[test]
fn true_path_buf() {
    let base = path::PathBuf::from("./");
    let rel = path::PathBuf::from("/etc/passwd");

    assert_eq!(Ok(true), base.is_path_trav(&rel))
}

#[test]
fn false_path() {
    let base = path::Path::new("./");
    let rel = path::Path::new("./tests/tests.rs");

    assert_eq!(Ok(false), base.is_path_trav(&rel))
}

#[test]
fn false_path_buf() {
    let base = path::PathBuf::from("./");
    let rel = path::PathBuf::from("./tests/tests.rs");

    assert_eq!(Ok(false), base.is_path_trav(&rel))
}

#[test]
fn no_file() {
    let base = path::PathBuf::from("./");
    let rel = path::PathBuf::from("./tests/weird_file");

    assert_eq!(Err(ErrorKind::NotFound), base.is_path_trav(&rel))
}
