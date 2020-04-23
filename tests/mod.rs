use std::panic;

use festivities::{fork, fork_id};

#[test]
fn fork_basically_works() {
    let res = fork("tests::fork_basically_works", fork_id!(), None, || {
        println!("works")
    })
    .unwrap();
    assert!(res.status.success());
}

#[test]
fn fork_passes_output() {
    let res = fork("tests::fork_passes_output", fork_id!(), None, || {
        println!("works")
    })
    .unwrap();
    assert!(res.status.success());

    let outstr = String::from_utf8_lossy(&res.stdout);
    assert!(outstr.contains("works"));
}
#[test]
fn fork_panic_works() {
    let res = fork("tests::fork_panic_works", fork_id!(), None, || {
        panic!("panicmsg")
    })
    .unwrap();
    assert!(!res.status.success());
    assert_eq!(70, res.status.code().unwrap());

    let outstr = String::from_utf8_lossy(&res.stderr);
    assert!(outstr.contains("panicmsg"));
}

#[test]
fn ids_distinct() {
    assert_ne!(fork_id!(), fork_id!());
}
