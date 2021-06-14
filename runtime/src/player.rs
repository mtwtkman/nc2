#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, PartialOrd, Ord)]
pub struct Player {
    id: usize,  // FIXME: implement an unique value generator.
}

impl Player {
    pub(crate) fn new(id: usize) -> Self {
        Self { id }
    }
}