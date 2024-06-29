//! BTree

#[derive(Clone)]
pub struct LeafNode<T: Clone> {
    key: usize,
    data: T,
}

#[derive(Clone)]
/// BTree internal node
pub struct InternalNode<T: Clone> {
    k: usize,
    keys: Vec<Option<usize>>,
    children: Vec<Option<InternalNode<T>>>,
    leaves: Vec<Option<LeafNode<T>>>,
}

/// BTree
pub struct BTree<T: Clone> {
    k: usize,
    root: InternalNode<T>,
}

impl<T: Clone> BTree<T> {

    /// Construct an empty `k`-dimensional BTree containing data of a specified type `T`
    pub fn new(k: usize) -> BTree<T> {
        BTree {k, root:InternalNode {
            k,
            keys: vec![None; k],
            children: vec![None; k],
            leaves: vec![None; k],
        }}
    }

}
