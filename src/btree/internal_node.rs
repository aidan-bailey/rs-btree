//! Internal Node

use super::leaf_node::LeafNode;

#[derive(Clone)]
/// BTree internal node
pub struct InternalNode<T: Clone> {
    k: usize,
    keys: Vec<usize>,
    children: Vec<InternalNode<T>>,
    leaves: Vec<LeafNode<T>>,
}

impl<T: Clone> InternalNode<T> {
    pub fn new(k: usize) -> InternalNode<T> {
        InternalNode {
            k,
            keys: Vec::new(),
            children: Vec::new(),
            leaves: Vec::new(),
        }
    }

    pub fn with_keyval(k: usize, key: usize, value: T) -> InternalNode<T> {
        let mut node = InternalNode::<T>::new(k);
        node.keys.push(key);
        node.leaves.push(LeafNode::new(key, value));
        node
    }
}
