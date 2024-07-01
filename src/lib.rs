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

    #[test]
    fn alphabet_test() {

        let mut btree = BTree::<char, char>::new(2);

        // Insert alphabet
        for c in vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'
        ] {
            let result = btree.insert(Record::new(c, c));
            assert!(result.is_ok(), "Result for {} is not ok: {}", c, result.err().unwrap());
        }

        // Root
        assert_eq!(3, btree.root.n());
        assert_eq!('b', btree.root.keys[0]);
        assert_eq!('d', btree.root.keys[1]);
        assert_eq!('f', btree.root.keys[2]);

        // Left
        assert_eq!(1, btree.root.children[0].n());
        assert_eq!('a', btree.root.children[0].keys[0]);

        // Middle
        assert_eq!(1, btree.root.children[1].n());
        assert_eq!('c', btree.root.children[1].keys[0]);

        // Right
        assert_eq!(1, btree.root.children[2].n());
        assert_eq!('e', btree.root.children[2].keys[0]);

        assert_eq!(2, btree.root.children[3].n());
        assert_eq!('g', btree.root.children[3].keys[0]);
        assert_eq!('h', btree.root.children[3].keys[1]);

    }
}
