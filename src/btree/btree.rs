//! BTree

use super::internal_node::InternalNode;

/// BTree
pub struct BTree<T: Clone> {
    k: usize,
    root: Option<InternalNode<T>>,
}

impl<T: Clone> BTree<T> {
    /// Dimensionality of BTree
    pub fn k(&self) -> usize {
        self.k
    }

    /// Construct an empty `k`-dimensional BTree containing value of a specified type `T`
    pub fn new(k: usize) -> BTree<T> {
        BTree { k, root: None }
    }

    /// Insert a new `value` of type `T` with a corresponding `key`
    pub fn insert(mut self, key: usize, value: T) {
        if self.root.is_none() {
            self.root = Some(InternalNode::<T>::with_keyval(self.k, key, value))
        }
    }
}
