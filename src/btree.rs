//! BTree

#[derive(Clone)]
pub struct LeafNode<T: Clone> {
    key: usize,
    value: T,
}

impl<T: Clone> LeafNode<T> {
    pub fn new(key: usize, value: T) -> LeafNode<T> {
        LeafNode { key, value }
    }
}

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

/// BTree
pub struct BTree<T: Clone> {
    k: usize,
    root: Option<InternalNode<T>>,
}

impl<T: Clone> BTree<T> {
    /// Dimensionality of BTree
    pub fn k(&self) -> usize {
        self.k
    }

    /// Construct an empty `k`-dimensional BTree containing value of a specified type `T`
    pub fn new(k: usize) -> BTree<T> {
        BTree { k, root: None }
    }

    /// Insert a new `value` of type `T` with a corresponding `key`
    pub fn insert(mut self, key: usize, value: T) {
        if self.root.is_none() {
            self.root = Some(InternalNode::<T>::with_keyval(self.k, key, value))
        }
    }
}
