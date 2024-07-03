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
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ] {
            let result = btree.insert(Record::new(c, c));
            assert!(
                result.is_ok(),
                "Result for {} is not ok: {}",
                c,
                result.err().unwrap()
            );
        }

        // Root
        assert!(!btree.root.leaf());
        assert_eq!(2, btree.root.n());
        assert_eq!('h', btree.root.keys[0]);
        assert_eq!('p', btree.root.keys[1]);

        // Left
        assert!(!btree.root.children[0].leaf());
        assert_eq!(1, btree.root.children[0].n());
        assert_eq!('d', btree.root.children[0].keys[0]);

        // Left Left
        assert!(!btree.root.children[0].children[0].leaf());
        assert_eq!(1, btree.root.children[0].children[0].n());
        assert_eq!('b', btree.root.children[0].children[0].keys[0]);

        // Left Left Left
        assert!(btree.root.children[0].children[0].children[0].leaf());
        assert_eq!(1, btree.root.children[0].children[0].children[0].n());
        assert_eq!('a', btree.root.children[0].children[0].children[0].keys[0]);

        // Left Left Right
        assert!(btree.root.children[0].children[0].children[1].leaf());
        assert_eq!(1, btree.root.children[0].children[0].children[1].n());
        assert_eq!('c', btree.root.children[0].children[0].children[1].keys[0]);

        // Left Right
        assert!(!btree.root.children[0].children[1].leaf());
        assert_eq!(1, btree.root.children[0].children[1].n());
        assert_eq!('f', btree.root.children[0].children[1].keys[0]);

        // Left Right Left
        assert!(btree.root.children[0].children[1].children[0].leaf());
        assert_eq!(1, btree.root.children[0].children[1].children[0].n());
        assert_eq!('e', btree.root.children[0].children[1].children[0].keys[0]);

        // Left Right Right
        assert!(btree.root.children[0].children[1].children[1].leaf());
        assert_eq!(1, btree.root.children[0].children[1].children[1].n());
        assert_eq!('g', btree.root.children[0].children[1].children[1].keys[0]);

        // Middle
        assert!(!btree.root.children[1].leaf());
        assert_eq!(1, btree.root.children[1].n());
        assert_eq!('l', btree.root.children[1].keys[0]);

        // Middle Left
        assert!(!btree.root.children[1].children[0].leaf());
        assert_eq!(1, btree.root.children[1].children[0].n());
        assert_eq!('j', btree.root.children[1].children[0].keys[0]);

        // Middle Left Left
        assert!(btree.root.children[1].children[0].children[0].leaf());
        assert_eq!(1, btree.root.children[1].children[0].children[0].n());
        assert_eq!('i', btree.root.children[1].children[0].children[0].keys[0]);

        // Middle Left Right
        assert!(btree.root.children[1].children[0].children[1].leaf());
        assert_eq!(1, btree.root.children[1].children[0].children[1].n());
        assert_eq!('k', btree.root.children[1].children[0].children[1].keys[0]);

        // Middle Right
        assert!(!btree.root.children[1].children[1].leaf());
        assert_eq!(1, btree.root.children[1].children[1].n());
        assert_eq!('n', btree.root.children[1].children[1].keys[0]);

        // Middle Right Left
        assert!(btree.root.children[1].children[1].children[0].leaf());
        assert_eq!(1, btree.root.children[1].children[1].children[0].n());
        assert_eq!('m', btree.root.children[1].children[1].children[0].keys[0]);

        // Middle Right Right
        assert!(btree.root.children[1].children[1].children[1].leaf());
        assert_eq!(1, btree.root.children[1].children[1].children[1].n());
        assert_eq!('o', btree.root.children[1].children[1].children[1].keys[0]);

        // Right
        assert!(!btree.root.children[2].leaf());
        assert_eq!(1, btree.root.children[2].n());
        assert_eq!('t', btree.root.children[2].keys[0]);

        // Right Left
        assert!(!btree.root.children[2].children[0].leaf());
        assert_eq!(1, btree.root.children[2].children[0].n());
        assert_eq!('r', btree.root.children[2].children[0].keys[0]);

        // Right Left Left
        assert!(btree.root.children[2].children[0].children[0].leaf());
        assert_eq!(1, btree.root.children[2].children[0].children[0].n());
        assert_eq!('q', btree.root.children[2].children[0].children[0].keys[0]);

        // Right Left Right
        assert!(btree.root.children[2].children[0].children[1].leaf());
        assert_eq!(1, btree.root.children[2].children[0].children[1].n());
        assert_eq!('s', btree.root.children[2].children[0].children[1].keys[0]);

        // Right Right
        assert!(!btree.root.children[2].children[1].leaf());
        assert_eq!(2, btree.root.children[2].children[1].n());
        assert_eq!('v', btree.root.children[2].children[1].keys[0]);
        assert_eq!('x', btree.root.children[2].children[1].keys[1]);

        // Right Right Left
        assert!(btree.root.children[2].children[1].children[0].leaf());
        assert_eq!(1, btree.root.children[2].children[1].children[0].n());
        assert_eq!('u', btree.root.children[2].children[1].children[0].keys[0]);

        // Right Right Middle
        assert!(btree.root.children[2].children[1].children[1].leaf());
        assert_eq!(1, btree.root.children[2].children[1].children[1].n());
        assert_eq!('w', btree.root.children[2].children[1].children[1].keys[0]);

        // Right Right Right
        assert!(btree.root.children[2].children[1].children[2].leaf());
        assert_eq!(2, btree.root.children[2].children[1].children[2].n());
        assert_eq!('y', btree.root.children[2].children[1].children[2].keys[0]);
        assert_eq!('z', btree.root.children[2].children[1].children[2].keys[1]);
    }
}
