pub mod btree;

#[cfg(test)]
mod tests {
    use crate::btree::btree::BTree;


    #[test]
    fn simple_btree() {
        let mut btree = BTree::<i32>::new(1);
        assert_eq!(1, btree.k());
        btree.insert(1, 4);

        let val = btree.find(&1);
        assert_eq!(4, val.unwrap());
    }

}
