#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
struct PlayerId(usize);
impl PlayerId {
    fn new() -> Self {
        use std::time::SystemTime;

        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        Self(seed.as_nanos() as usize)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
pub(crate) struct Player {
    id: PlayerId,
}
impl Player {
    pub(crate) fn new() -> Self {
        let id = PlayerId::new();
        Self { id }
    }
}
