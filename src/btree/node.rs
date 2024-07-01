//! Internal Node

use super::record::Record;

#[derive(Clone, Debug)]
/// BTree internal node
pub struct Node<T: Clone> {
    t: usize,
    pub(crate) keys: Vec<usize>,
    pub(crate) records: Vec<Record<T>>,
    pub(crate) children: Vec<Node<T>>,
}

impl<T: Clone> Node<T> {
    pub fn new(t: usize) -> Node<T> {
        Node {
            t,
            keys: Vec::<usize>::new(),
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

    pub fn full(&self) -> bool {
        self.n() == self.t * 2 - 1
    }

    pub fn empty(&self) -> bool {
        self.keys.is_empty()
    }

    pub fn with_record(k: usize, record: Record<T>) -> Node<T> {
        let mut node = Node::<T>::new(k);
        node.keys.push(record.key());
        node.records.push(record);
        node
    }

    pub(crate) fn split_child(&mut self, i: usize) -> Result<(), &'static str> {
        if self.full() {
            return Err("No space for child's key");
        }

        if self.children.get(i).is_none() {
            return Err("Child does not exist");
        }

        let t = self.t;

        // construct new child node
        let mut z = Node::<T>::new(self.t);

        // scope for borrowing a mutable y
        {
            // get child to split
            let y = &mut self.children[i];

            // z gets ys greatest keys
            for _j in 0..(t - 1) {
                if let Some(key) = y.keys.pop() {
                    z.keys.insert(0, key)
                } else {
                    return Err("Missing key");
                }
            }

            // move greatest children
            if !y.leaf() {
                for _j in 0..(t) {
                    if let Some(child) = y.children.pop() {
                        z.children.insert(0, child)
                    } else {
                        return Err("Missing child");
                    }
                }
            }

            // move median key to parent
            if let Some(key) = y.keys.pop() {
                if (i + 1) >= self.n() {
                    self.keys.push(key)
                } else {
                    self.keys.insert(i + 1, key)
                }
            } else {
                return Err("Missing key");
            }
        }

        // insert z as child

        if (i + 1) >= self.n() {
            self.children.push(z);
        } else {
            self.children.insert(i + 1, z);
        }

        Ok(())
    }

    pub(crate) fn insert_nonfull(&mut self, record: Record<T>) -> Result<(), &'static str> {
        if self.full() {
            return Err("Node is full");
        }

        let mut i = 0;

        if self.leaf() {
            if self.empty() {
                self.keys.push(record.key());
                self.records.push(record)
            } else {
                while i < self.n() && record.key() >= self.keys[i] {
                    i += 1
                }
                if i >= self.n() {
                    self.keys.push(record.key());
                    self.records.push(record);
                } else {
                    self.keys.insert(i, record.key());
                    self.records.insert(i, record);
                }
            }
        } else {
            while i < self.n() && record.key() >= self.keys[i] {
                i += 1
            }
            if i != self.n() && self.children[i].full() {
                let _ = self.split_child(i);
                if record.key() > self.keys[i] {
                    i += 1
                }
            }
            return self.children[i].insert_nonfull(record);
        }

        Ok(())
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
