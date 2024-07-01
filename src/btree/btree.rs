//! BTree

use super::{
    node::Node, record::Record,
};

/// BTree
pub struct BTree<T: Clone> {
    k: usize,
    root: Node<T>,
}

impl<T: Clone> BTree<T> {
    /// Dimensionality of BTree
    pub fn k(&self) -> usize {
        self.k
    }

    /// Construct an empty `k`-dimensional BTree containing value of a specified type `T`
    pub fn new(k: usize) -> BTree<T> {
        BTree::<T> { k, root: Node::<T>::new(k) }
    }

    fn split_root(&mut self) -> Result<(), &'static str> {

        if !self.root.leaf() {
            return Err("Root node is not a leaf node");
        }

        // construct new child of root
        let mut s = Node::<T>::new(self.k);

        // move all of roots keys
        while !self.root.keys.is_empty() {
             if let Some(key) = self.root.keys.pop() {
                s.keys.push(key);
            }
        }

        // move all of roots records
        while !self.root.records.is_empty() {
            if let Some(record) = self.root.records.pop() {
                s.records.push(record);
            }
        }

        // assign new child to root
        self.root.children.push(s);

        // split child
        self.root.split_child(0)
    }

    /// Insert a new `value` of type `T` with a corresponding `key`
    pub fn insert(&mut self, record: Record<T>) -> Result<(), &'static str> {
        self.root.insert(record)
    }

    pub fn search(&self, key: usize) -> Result<Option<(&Node<T>, usize)>, &'static str> {
        self.root.search(key)
    }
}
