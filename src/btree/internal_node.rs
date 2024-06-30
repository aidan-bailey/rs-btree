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
                                if let BTreeNode::Leaf(leaf) = child {
                                    return leaf.find(key);
                                } else if let BTreeNode::Internal(internal) = child {
                                    return internal.find(key);
                                }
                            }
                            None => return Ok(None),
                        }
                    } else {
                        continue;
                    }
                }
                None => {
                    if let Some(child) = &self.children[i] {
                        if let BTreeNode::Leaf(leaf) = child {
                            return leaf.find(key);
                        } else if let BTreeNode::Internal(internal) = child {
                            return internal.find(key);
                        }
                    } else {
                        return Ok(None);
                    }
                }
            }
        }

        if let Some(child) = &self.children[self.k] {
            if let BTreeNode::Leaf(leaf) = child {
                return leaf.find(key);
            } else if let BTreeNode::Internal(internal) = child {
                return internal.find(key);
            }
        }

        Ok(None)
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
