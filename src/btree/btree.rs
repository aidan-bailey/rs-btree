//! BTree

use super::{
    node::Node, record::Record,
};

/// BTree
#[derive(Debug)]
pub struct BTree<KT: Ord + Copy, DT: Clone> {
    t: usize,
    pub(crate) root: Node<KT, DT>,
}

impl<KT: Ord + Copy, DT: Clone> BTree<KT, DT> {
    /// minimum degree of BTree
    pub fn t(&self) -> usize {
        self.t
    }

    /// Construct an empty BTree with a minimum degree `t` containing value of a specified type `T`
    pub fn new(t: usize) -> BTree<KT, DT> {
        BTree::<KT, DT> { t, root: Node::<KT, DT>::new(t) }
    }

    fn split_root(&mut self) -> Result<(), &'static str> {

        if !self.root.leaf() {
            return Err("Root node is not a leaf node");
        }

        // construct new child of root
        let mut s = Node::<KT, DT>::new(self.t);

        // move all of roots keys
        while !self.root.keys.is_empty() {
             if let Some(key) = self.root.keys.pop() {
                s.keys.insert(0, key);
            }
        }

        // move all of roots records
        while !self.root.records.is_empty() {
            if let Some(record) = self.root.records.pop() {
                s.records.insert(0, record);
            }
        }

        // assign new child to root
        self.root.children.push(s);

        // split child
        self.root.split_child(0)
    }

    /// Insert a new `value` of type `T` with a corresponding `key`
    pub fn insert(&mut self, record: Record<KT, DT>) -> Result<(), &'static str> {
        if self.root.full() {
            let result = self.split_root();
            result?;
        }
        self.root.insert_nonfull(record)
    }

    pub fn search(&self, key: KT) -> Result<Option<(&Node<KT, DT>, usize)>, &'static str> {
        self.root.search(key)
    }
}
