use std::{io::Read, thread};

use super::{fork::*, *};

fn sleep(ms: u64) {
    thread::sleep(::std::time::Duration::from_millis(ms));
}

fn capturing_output(cmd: &mut process::Command) {
    // Only actually capture stdout since we can't use
    // wait_with_output() since it for some reason consumes the `Child`.
    cmd.stdout(process::Stdio::piped())
        .stderr(process::Stdio::inherit());
}

fn inherit_output(cmd: &mut process::Command) {
    cmd.stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit());
}

fn wait_for_child_output(child: &mut ChildWrapper, _file: &mut fs::File) -> String {
    let mut output = String::new();
    child
        .inner_mut()
        .stdout
        .as_mut()
        .unwrap()
        .read_to_string(&mut output)
        .unwrap();
    assert!(child.wait().unwrap().success());
    output
}

fn wait_for_child(child: &mut ChildWrapper, _file: &mut fs::File) {
    assert!(child.wait().unwrap().success());
}

#[test]
fn fork_basically_works() {
    let status = fork(
        "fork::test::fork_basically_works",
        rusty_fork_id!(),
        |_| (),
        |child, _| child.wait().unwrap(),
        || println!("hello from child"),
    )
    .unwrap();
    assert!(status.success());
}

#[test]
fn child_output_captured_and_repeated() {
    let output = fork(
        "fork::test::child_output_captured_and_repeated",
        rusty_fork_id!(),
        capturing_output,
        wait_for_child_output,
        || {
            fork(
                "fork::test::child_output_captured_and_repeated",
                rusty_fork_id!(),
                |_| (),
                wait_for_child,
                || println!("hello from child"),
            )
            .unwrap()
        },
    )
    .unwrap();
    assert!(output.contains("hello from child"));
}

#[test]
fn child_killed_if_parent_exits_first() {
    let output = fork(
        "fork::test::child_killed_if_parent_exits_first",
        rusty_fork_id!(),
        capturing_output,
        wait_for_child_output,
        || {
            fork(
                "fork::test::child_killed_if_parent_exits_first",
                rusty_fork_id!(),
                inherit_output,
                |_, _| (),
                || {
                    sleep(1_000);
                    println!("hello from child");
                },
            )
            .unwrap()
        },
    )
    .unwrap();

    sleep(2_000);
    assert!(
        !output.contains("hello from child"),
        "Had unexpected output:\n{}",
        output
    );
}

#[test]
fn child_killed_if_parent_panics_first() {
    let output = fork(
        "fork::test::child_killed_if_parent_panics_first",
        rusty_fork_id!(),
        capturing_output,
        wait_for_child_output,
        || {
            assert!(panic::catch_unwind(panic::AssertUnwindSafe(|| fork(
                "fork::test::child_killed_if_parent_panics_first",
                rusty_fork_id!(),
                inherit_output,
                |_, _| panic!("testing a panic, nothing to see here"),
                || {
                    sleep(1_000);
                    println!("hello from child");
                }
            )
            .unwrap()))
            .is_err());
        },
    )
    .unwrap();

    sleep(2_000);
    assert!(
        !output.contains("hello from child"),
        "Had unexpected output:\n{}",
        output
    );
}

#[test]
fn child_aborted_if_panics() {
    let status = fork(
        "fork::test::child_aborted_if_panics",
        rusty_fork_id!(),
        |_| (),
        |child, _| child.wait().unwrap(),
        || panic!("testing a panic, nothing to see here"),
    )
    .unwrap();
    assert_eq!(70, status.code().unwrap());
}
