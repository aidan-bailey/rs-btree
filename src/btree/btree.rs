//! BTree

use super::{
    btree_node::BTreeNode, internal_node::InternalNode, leaf_node::LeafNode, node::Node,
    record::Record,
};

/// BTree
pub struct BTree<T: Clone> {
    k: usize,
    root: Option<BTreeNode<T>>,
}

impl<T: Clone> BTree<T> {
    /// Dimensionality of BTree
    pub fn k(&self) -> usize {
        self.k
    }

    /// Construct an empty `k`-dimensional BTree containing value of a specified type `T`
    pub fn new(k: usize) -> BTree<T> {
        let btree = BTree::<T> { k, root: None };
        btree
    }

    /// Insert a new `value` of type `T` with a corresponding `key`
    pub fn insert(&mut self, record: Record<T>) {
        if self.root.is_none() {
            self.root = Some(BTreeNode::Leaf(LeafNode::<T>::with_record(self.k, record)))
        }
    }

    pub fn find(&self, key: &usize) -> Result<Option<&Record<T>>, &'static str> {
        if let Some(node) = &self.root {
            match node {
                BTreeNode::Internal(node) => return node.find(key),
                BTreeNode::Leaf(node) => return node.find(key),
            }
        }
        Ok(None)
    }
}
