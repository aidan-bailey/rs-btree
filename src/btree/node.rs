//! Internal Node

use super::{record::Record};

#[derive(Clone)]
/// BTree internal node
pub struct Node<T: Clone> {
    k: usize,
    keys: Vec<Option<usize>>,
    records: Vec<Option<Record<T>>>,
    children: Vec<Option<Node<T>>>,
}

impl<T: Clone> Node<T> {
    pub fn new(k: usize) -> Node<T> {
        Node {
            k,
            keys: vec![None; k],
            records: vec![None; k],
            children: vec![None; k + 1],
        }
    }

    pub fn with_record(k: usize, record: Record<T>) -> Node<T> {
        let mut node = Node::<T>::new(k);
        node.keys[0] = Some(record.key().clone());
        node.records[0] = Some(record);
        node
    }

    pub fn find(&self, key: &usize) -> Result<Option<&Record<T>>, &'static str> {
        for i in 0..self.k {
            match &self.keys[i] {
                Some(nkey) => {
                    if key == nkey {
                        match &self.records[i] {
                            Some(record) => return Ok(Some(record)),
                            None => return Err("Record is missing"),
                        }
                    } else if key < nkey {
                        match &self.children[i] {
                            Some(child) => {
                                return child.find(key);
                            }
                            None => return Ok(None),
                        }
                    } else {
                        continue;
                    }
                }
                None => {
                    if let Some(child) = &self.children[i] {
                        return child.find(key);
                    } else {
                        return Ok(None);
                    }
                }
            }
        }

        if let Some(child) = &self.children[self.k] {
            return child.find(key);
        }

        Ok(None)
    }

}
