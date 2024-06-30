//! Leaf Node

use super::{node::Node, record::Record};

#[derive(Clone)]
pub struct LeafNode<T: Clone> {
    k: usize,
    keys: Vec<Option<usize>>,
    records: Vec<Option<Record<T>>>,
}

impl<T: Clone> Node<T> for LeafNode<T> {
    fn find(&self, key: &usize) -> Result<Option<&Record<T>>, &'static str> {
        for i in 0..self.k {
            match &self.keys[i] {
                Some(nkey) => {
                    if nkey == key {
                        match &self.records[i] {
                            Some(value) => return Ok(Some(value)),
                            _ => return Err("Value missing"),
                        }
                    }
                }
                _ => return Ok(None),
            }
        }

        return Ok(None);
    }
}

impl<T: Clone> LeafNode<T> {
    /// Construct a new leaf node with the given key and value
    pub fn new(k: usize) -> LeafNode<T> {
        LeafNode {
            k,
            keys: vec![None; k],
            records: vec![None; k],
        }
    }

    /// Construct a new leaf node with the given key and value
    pub fn with_record(k: usize, record: Record<T>) -> LeafNode<T> {
        let mut leaf = LeafNode::new(k);
        leaf.keys[0] = Some(record.key().clone());
        leaf.records[0] = Some(record);
        leaf
    }
}
