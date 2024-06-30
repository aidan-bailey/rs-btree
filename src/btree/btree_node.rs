use super::{internal_node::InternalNode, leaf_node::LeafNode};

#[derive(Clone)]
pub enum BTreeNode<T: Clone> {
    Internal(InternalNode<T>),
    Leaf(LeafNode<T>),
}
