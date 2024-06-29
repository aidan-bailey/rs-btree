//! Leaf Node

#[derive(Clone)]
pub struct LeafNode<T: Clone> {
    key: usize,
    value: T,
}

impl<T: Clone> LeafNode<T> {

    /// Construct a new leaf node with the given key and value
    pub fn new(key: usize, value: T) -> LeafNode<T> {
        LeafNode { key, value }
    }

    /// Get the key
    pub fn key(self) -> usize {
        self.key
    }

    /// Get the value
    pub fn value(self) -> T {
        self.value.clone()
    }

    /// Peak at the value
    pub fn peak(&self) -> &T {
        &self.value
    }

}
