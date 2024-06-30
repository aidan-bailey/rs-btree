pub mod btree;

#[cfg(test)]
mod tests {
    use crate::btree::{btree::BTree, record::Record};


    #[test]
    fn simple_btree() {
        let mut btree = BTree::<i32>::new(1);
        let record = Record::new(1, 1);
        assert_eq!(1, btree.k());
        btree.insert(record);
        let result = btree.find(&1);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
        assert_eq!(1, result.unwrap().unwrap().key().clone());
        let result2 = btree.find(&2);
        assert!(result2.is_ok());
        assert!(result2.unwrap().is_none());
    }

}
