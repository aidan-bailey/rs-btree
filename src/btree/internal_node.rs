//! Internal Node

use super::leaf_node::LeafNode;

#[derive(Clone)]
/// BTree internal node
pub struct InternalNode<T: Clone> {
    k: usize,
    keys: Vec<Option<usize>>,
    children: Vec<Option<InternalNode<T>>>,
    leaves: Vec<Option<LeafNode<T>>>,
}

impl<T: Clone> InternalNode<T> {
    pub fn new(k: usize) -> InternalNode<T> {
        InternalNode {
            k,
            keys: vec![None; k],
            children: vec![None; k],
            leaves: vec![None; k],
        }
    }

    pub fn with_keyval(k: usize, key: usize, value: T) -> InternalNode<T> {
        let mut node = InternalNode::<T>::new(k);
        node.keys[0] = Some(key);
        node.leaves[0] = Some(LeafNode::<T>::new(key, value));
        node
    }

}
