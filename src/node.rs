use std::hash::Hash;
#[derive(Debug)]
pub struct Node<T>
where
    T: Copy + Eq + Hash + ?Sized,
{
    pub identifier: T,
}

// TODO: write comments why these trait bounds are needed.
impl<T: Copy + Eq + Hash + ?Sized> Node<T> {
    pub fn new(identifier: T) -> Self {
        Self { identifier }
    }
}
