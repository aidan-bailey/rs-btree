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
            children: vec![None; k + 1],
            leaves: vec![None; k],
        }
    }

    pub fn with_keyval(k: usize, key: usize, value: T) -> InternalNode<T> {
        let mut node = InternalNode::<T>::new(k);
        node.keys[0] = Some(key);
        node.leaves[0] = Some(LeafNode::<T>::new(key, value));
        node
    }

    pub fn find(&self, key: &usize) -> Option<T> {
        for i in 0..self.k {
            match &self.keys[i] {
                Some(nkey) => {
                    if key == nkey {
                        match &self.leaves[i] {
                            Some(leaf) => return Some(leaf.value().clone()),
                            _ => panic!("Missing leaf"),
                        }
                    } else if key < nkey {
                        match &self.children[i] {
                            Some(child) => return child.find(key),
                            _ => return None,
                        }
                    } else {
                        continue;
                    }
                }
                _ => match &self.children[i] {
                    Some(child) => return child.find(key),
                    _ => return None,
                },
            }
        }

        if let Some(child) = &self.children[self.k] {
            return child.find(key);
        }

        None
    }
}
