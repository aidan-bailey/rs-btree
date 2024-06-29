pub mod btree;

#[cfg(test)]
mod tests {

    use crate::btree::BTree;

    #[test]
    fn construct_empty_btree() {
        let btree = BTree::<i32>::new(1);
        assert_eq!(1, btree.k())
    }
}
