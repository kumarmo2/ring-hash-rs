use std::hash::Hash;
#[derive(Debug)]
pub struct Node<'identifier, T>
where
    T: Eq + Hash + ?Sized,
{
    pub identifier: &'identifier T,
}

impl<'identifier, T: Eq + Hash + ?Sized> Node<'identifier, T> {
    /*
     * Eq + Hash are needed on the `T` as that is treated as the "identity" of the Node
     * */
    pub fn new(identifier: &'identifier T) -> Self {
        Self { identifier }
    }
}
