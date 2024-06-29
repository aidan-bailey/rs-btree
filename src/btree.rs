//! BTree

// BTree leaf node
pub struct LeafNode<T> {
    key: usize,
    data: T
}

// BTree internal node
pub struct InternalNode<T> {
    k: usize,
    keys: Vec<usize>,
    children: Vec<Self>,
    leaves: Vec<LeafNode<T>>,
}

// BTree
pub struct BTree<T> {
    k: usize,
    root: InternalNode<T>
}
