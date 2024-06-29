//! Leaf Node

#[derive(Clone)]
pub struct LeafNode<T: Clone> {
    key: usize,
    value: T,
}

impl<T: Clone> LeafNode<T> {
    pub fn new(key: usize, value: T) -> LeafNode<T> {
        LeafNode { key, value }
    }
}
