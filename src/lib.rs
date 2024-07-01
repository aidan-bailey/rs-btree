pub mod btree;

#[cfg(test)]
mod tests {
    use crate::btree::{btree::BTree, record::Record};


    #[test]
    fn simple_btree() {
        let mut btree = BTree::<i32>::new(1);
        let record = Record::new(1, 1);
        assert_eq!(1, btree.k());
    }

}
