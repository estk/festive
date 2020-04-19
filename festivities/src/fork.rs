use std::{
    env,
    hash::{Hash, Hasher},
    io, panic, process,
    time::Duration,
};

use process_control::{ChildExt, Timeout};

use crate::{cmdline, Result};

const OCCURS_ENV: &str = "FESTIVE_FORK_OCCURS";
const OCCURS_TERM_LENGTH: usize = 17; /* ':' plus 16 hexits */

pub fn fork<ID, CHILD>(
    test_path: &str,
    fork_id: ID,
    timeout: Option<u64>,
    in_child: CHILD,
) -> Result<process_control::Output>
where
    ID: Hash,
    CHILD: FnOnce(),
{
    let test_name = test_path
        .find("::")
        .map(|ix| &test_path[ix + 2..])
        .unwrap_or(&test_path);

    let fork_id = id_str(fork_id);

    // Erase the generics so we don't instantiate the actual implementation for
    // every single test
    let mut in_child = Some(in_child);

    fork_impl(test_name, fork_id, timeout.unwrap_or(10), &mut || {
        in_child.take().unwrap()()
    })
}

fn fork_impl(
    test_name: &str,
    fork_id: String,
    timeout: u64,
    in_child: &mut dyn FnMut(),
) -> Result<process_control::Output> {
    let mut occurs = env::var(OCCURS_ENV).unwrap_or_else(|_| String::new());

    // Were in the targeted fork
    if occurs.contains(&fork_id) {
        match panic::catch_unwind(panic::AssertUnwindSafe(in_child)) {
            Ok(_) => process::exit(0),
            // Assume that the default panic handler already printed something
            //
            // We don't use process::abort() since it produces core dumps on
            // some systems and isn't something more special than a normal
            // panic.
            Err(_) => process::exit(70 /* EX_SOFTWARE */),
        }
    } else {
        // Prevent misconfiguration creating a fork bomb
        if occurs.len() > 16 * OCCURS_TERM_LENGTH {
            panic!("festivities: Not forking due to >=16 levels of recursion");
        }

        occurs.push_str(&fork_id);

        let mut command =
            process::Command::new(env::current_exe().expect("current_exe() failed, cannot fork"));

        command
            .args(cmdline::strip_cmdline(env::args())?)
            .args(cmdline::RUN_TEST_ARGS)
            .arg(test_name)
            .env(OCCURS_ENV, &occurs)
            .stdin(process::Stdio::null())
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::piped());

        let out = command
            .spawn()?
            .with_output_timeout(Duration::from_millis(timeout))
            .terminating()
            .wait()?
            .ok_or_else(|| io::Error::new(io::ErrorKind::TimedOut, "Process timed out"))?;

        Ok(out)
    }
}

fn id_str<ID: Hash>(id: ID) -> String {
    let mut hasher = fnv::FnvHasher::default();
    id.hash(&mut hasher);

    return format!(":{:016X}", hasher.finish());
}

#[macro_export]
macro_rules! fork_id {
    () => {{
        struct _ForkId;
        $crate::ForkId::of(::std::any::TypeId::of::<_ForkId>())
    }};
}

#[derive(Clone, Hash, PartialEq, Debug)]
pub struct ForkId(::std::any::TypeId);
impl ForkId {
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub fn of(id: ::std::any::TypeId) -> Self {
        ForkId(id)
    }
}
