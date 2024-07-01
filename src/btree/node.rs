//! Internal Node

use super::record::Record;

#[derive(Clone)]
/// BTree internal node
pub struct Node<T: Clone> {
    k: usize,
    keys: Vec<usize>,
    records: Vec<Record<T>>,
    children: Vec<Node<T>>,
}

impl<T: Clone> Node<T> {
    pub fn new(k: usize) -> Node<T> {
        Node {
            k,
            keys: vec![],
            records: vec![],
            children: vec![],
        }
    }

    /// Whether or not the node is a leaf
    pub fn leaf(&self) -> bool {
        self.children.is_empty()
    }

    /// Number of keys contained in the node
    pub fn n(&self) -> usize {
        self.keys.len()
    }

    pub fn with_record(k: usize, record: Record<T>) -> Node<T> {
        let mut node = Node::<T>::new(k);
        node.keys.push(record.key().clone());
        node.records.push(record);
        node
    }

    pub fn search(&self, key: usize) -> Result<Option<(&Node<T>, usize)>, &'static str> {

        // initialise the index
        let mut i: usize = 0;

        // search through the node for a key that is
        // greater than or equal to the searched for key
        while i < self.n() && key > self.keys[i] {
            i += 1
        }

        if i < self.n() && key == self.keys[i] {
            // an equal key has been found
            return Ok(Some((self, i)));
        } else if self.leaf() {
            // a greater than key has been found but
            // the current node is a leaf
            return Ok(None);
        }

        // a greater than key has been found,
        // we must continue down the tree
        self.children[i].search(key)

    }
}
