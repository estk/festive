#[macro_export]
macro_rules! rusty_fork_id {
    () => {{
        struct _ForkId;
        $crate::ForkId::of(::std::any::TypeId::of::<_RustyForkId>())
    }};
}

/// The type of the value produced by
/// [`rusty_fork_id!`](macro.rusty_fork_id.html).
#[derive(Clone, Hash, PartialEq, Debug)]
pub struct ForkId(::std::any::TypeId);
impl ForkId {
    #[allow(missing_docs)]
    #[doc(hidden)]
    pub fn of(id: ::std::any::TypeId) -> Self {
        ForkId(id)
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn ids_are_actually_distinct() {
        assert_ne!(rusty_fork_id!(), rusty_fork_id!());
    }
}
