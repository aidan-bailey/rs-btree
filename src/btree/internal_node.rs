//! Internal Node

use super::{btree_node::BTreeNode, leaf_node::LeafNode, node::Node, record::Record};

#[derive(Clone)]
/// BTree internal node
pub struct InternalNode<T: Clone> {
    k: usize,
    keys: Vec<Option<usize>>,
    records: Vec<Option<Record<T>>>,
    children: Vec<Option<BTreeNode<T>>>,
}

impl<T: Clone> Node<T> for InternalNode<T> {
    fn find(&self, key: &usize) -> Result<Option<&Record<T>>, &'static str> {
        todo!()
    }
}

impl<T: Clone> InternalNode<T> {
    pub fn new(k: usize) -> InternalNode<T> {
        InternalNode {
            k,
            keys: vec![None; k],
            records: vec![None; k],
            children: vec![None; k + 1],
        }
    }
}
