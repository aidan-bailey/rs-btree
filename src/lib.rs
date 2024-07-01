pub mod btree;

#[cfg(test)]
mod tests {
    use crate::btree::{btree::BTree, record::Record};

    #[test]
    fn simple_insert() {
        // Insert 1 -> 2 -> 3 -> 4

        let mut btree = BTree::<i32, i32>::new(2);
        assert_eq!(2, btree.t());
        assert!(btree.root.leaf());

        // Insert 1
        let result = btree.insert(Record::new(1, 1));
        assert!(result.is_ok());
        assert_eq!(1, btree.root.keys.len());
        assert_eq!(1, btree.root.keys[0]);
        assert!(btree.root.leaf());

        // Insert 3
        let result = btree.insert(Record::new(3, 3));
        assert!(result.is_ok());
        assert_eq!(2, btree.root.keys.len());
        assert_eq!(1, btree.root.keys[0]);
        assert_eq!(3, btree.root.keys[1]);
        assert!(btree.root.leaf());

        // Insert 2
        let result = btree.insert(Record::new(2, 2));
        assert!(result.is_ok());
        assert_eq!(3, btree.root.keys.len());
        assert_eq!(1, btree.root.keys[0]);
        assert_eq!(2, btree.root.keys[1]);
        assert_eq!(3, btree.root.keys[2]);
        assert!(btree.root.leaf());

        // Insert 4
        let result = btree.insert(Record::new(4, 4));
        assert!(result.is_ok());
        // ROOT
        assert!(!btree.root.leaf());
        assert_eq!(1, btree.root.keys.len());
        assert_eq!(2, btree.root.keys[0]);
        assert_eq!(2, btree.root.children.len());
        //LEFT
        assert!(btree.root.children[0].leaf());
        assert_eq!(1, btree.root.children[0].keys.len());
        assert_eq!(1, btree.root.children[0].keys[0]);
        // RIGHT
        assert!(btree.root.children[1].leaf());
        assert_eq!(2, btree.root.children[1].keys.len());
        assert_eq!(3, btree.root.children[1].keys[0]);
        assert_eq!(4, btree.root.children[1].keys[1]);

    }

}
