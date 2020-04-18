fn main() {}

/// These are all good if they have unique pids
#[cfg(test)]
mod test {
    use festive::festive;
    use std::process;
    #[festive]
    fn nested() {
        println!("Forked: My pid={}", process::id());
    }

    #[festive]
    fn forked() {
        println!("Forked: My pid={}", process::id());
    }

    #[festive(timeout_ms = 10)]
    #[no_mangle]
    fn forked_timeout() {
        println!("Forked: My pid={}", process::id());
    }

    #[festive(timeout_ms = 10)]
    #[test]
    fn forked_timeout_double_test() {
        println!("Forked: My pid={}", process::id());
    }

    /// We can still should_panic
    #[festive]
    #[should_panic]
    fn forked_panic() {
        println!("Forked: My pid={}", process::id());
        panic!("this is an expected panic message")
    }

    #[test]
    fn normal1() {
        println!("Normal1: My pid={}", process::id());
    }
}
