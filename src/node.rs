use std::hash::Hash;
#[derive(Debug)]
pub struct Node<'identifier, T>
where
    T: Eq + Hash + ?Sized,
{
    pub identifier: &'identifier T,
}

// TODO: write comments why these trait bounds are needed.
impl<'identifier, T: Eq + Hash + ?Sized> Node<'identifier, T> {
    pub fn new(identifier: &'identifier T) -> Self {
        Self { identifier }
    }
}
