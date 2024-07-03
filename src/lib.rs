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
        let mut btree2 = BTree::<char, char>::new(2);
        let mut btree3 = BTree::<char, char>::new(3);

        // Insert alphabet
        for c in vec![
            'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q',
            'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
        ] {
            let result = btree2.insert(Record::new(c, c));
            assert!(
                result.is_ok(),
                "Btree2 Result for {} is not ok: {}",
                c,
                result.err().unwrap()
            );
            let result = btree3.insert(Record::new(c, c));
            assert!(
                result.is_ok(),
                "Btree3 result for {} is not ok: {}",
                c,
                result.err().unwrap()
            );
        }

        // Check BTree with Max. Degree 4

        // Root
        assert!(!btree2.root.leaf());
        assert_eq!(2, btree2.root.n());
        assert_eq!('h', btree2.root.keys[0]);
        assert_eq!('p', btree2.root.keys[1]);

        // Left
        assert!(!btree2.root.children[0].leaf());
        assert_eq!(1, btree2.root.children[0].n());
        assert_eq!('d', btree2.root.children[0].keys[0]);

        // Left Left
        assert!(!btree2.root.children[0].children[0].leaf());
        assert_eq!(1, btree2.root.children[0].children[0].n());
        assert_eq!('b', btree2.root.children[0].children[0].keys[0]);

        // Left Left Left
        assert!(btree2.root.children[0].children[0].children[0].leaf());
        assert_eq!(1, btree2.root.children[0].children[0].children[0].n());
        assert_eq!('a', btree2.root.children[0].children[0].children[0].keys[0]);

        // Left Left Right
        assert!(btree2.root.children[0].children[0].children[1].leaf());
        assert_eq!(1, btree2.root.children[0].children[0].children[1].n());
        assert_eq!('c', btree2.root.children[0].children[0].children[1].keys[0]);

        // Left Right
        assert!(!btree2.root.children[0].children[1].leaf());
        assert_eq!(1, btree2.root.children[0].children[1].n());
        assert_eq!('f', btree2.root.children[0].children[1].keys[0]);

        // Left Right Left
        assert!(btree2.root.children[0].children[1].children[0].leaf());
        assert_eq!(1, btree2.root.children[0].children[1].children[0].n());
        assert_eq!('e', btree2.root.children[0].children[1].children[0].keys[0]);

        // Left Right Right
        assert!(btree2.root.children[0].children[1].children[1].leaf());
        assert_eq!(1, btree2.root.children[0].children[1].children[1].n());
        assert_eq!('g', btree2.root.children[0].children[1].children[1].keys[0]);

        // Middle
        assert!(!btree2.root.children[1].leaf());
        assert_eq!(1, btree2.root.children[1].n());
        assert_eq!('l', btree2.root.children[1].keys[0]);

        // Middle Left
        assert!(!btree2.root.children[1].children[0].leaf());
        assert_eq!(1, btree2.root.children[1].children[0].n());
        assert_eq!('j', btree2.root.children[1].children[0].keys[0]);

        // Middle Left Left
        assert!(btree2.root.children[1].children[0].children[0].leaf());
        assert_eq!(1, btree2.root.children[1].children[0].children[0].n());
        assert_eq!('i', btree2.root.children[1].children[0].children[0].keys[0]);

        // Middle Left Right
        assert!(btree2.root.children[1].children[0].children[1].leaf());
        assert_eq!(1, btree2.root.children[1].children[0].children[1].n());
        assert_eq!('k', btree2.root.children[1].children[0].children[1].keys[0]);

        // Middle Right
        assert!(!btree2.root.children[1].children[1].leaf());
        assert_eq!(1, btree2.root.children[1].children[1].n());
        assert_eq!('n', btree2.root.children[1].children[1].keys[0]);

        // Middle Right Left
        assert!(btree2.root.children[1].children[1].children[0].leaf());
        assert_eq!(1, btree2.root.children[1].children[1].children[0].n());
        assert_eq!('m', btree2.root.children[1].children[1].children[0].keys[0]);

        // Middle Right Right
        assert!(btree2.root.children[1].children[1].children[1].leaf());
        assert_eq!(1, btree2.root.children[1].children[1].children[1].n());
        assert_eq!('o', btree2.root.children[1].children[1].children[1].keys[0]);

        // Right
        assert!(!btree2.root.children[2].leaf());
        assert_eq!(1, btree2.root.children[2].n());
        assert_eq!('t', btree2.root.children[2].keys[0]);

        // Right Left
        assert!(!btree2.root.children[2].children[0].leaf());
        assert_eq!(1, btree2.root.children[2].children[0].n());
        assert_eq!('r', btree2.root.children[2].children[0].keys[0]);

        // Right Left Left
        assert!(btree2.root.children[2].children[0].children[0].leaf());
        assert_eq!(1, btree2.root.children[2].children[0].children[0].n());
        assert_eq!('q', btree2.root.children[2].children[0].children[0].keys[0]);

        // Right Left Right
        assert!(btree2.root.children[2].children[0].children[1].leaf());
        assert_eq!(1, btree2.root.children[2].children[0].children[1].n());
        assert_eq!('s', btree2.root.children[2].children[0].children[1].keys[0]);

        // Right Right
        assert!(!btree2.root.children[2].children[1].leaf());
        assert_eq!(2, btree2.root.children[2].children[1].n());
        assert_eq!('v', btree2.root.children[2].children[1].keys[0]);
        assert_eq!('x', btree2.root.children[2].children[1].keys[1]);

        // Right Right Left
        assert!(btree2.root.children[2].children[1].children[0].leaf());
        assert_eq!(1, btree2.root.children[2].children[1].children[0].n());
        assert_eq!('u', btree2.root.children[2].children[1].children[0].keys[0]);

        // Right Right Middle
        assert!(btree2.root.children[2].children[1].children[1].leaf());
        assert_eq!(1, btree2.root.children[2].children[1].children[1].n());
        assert_eq!('w', btree2.root.children[2].children[1].children[1].keys[0]);

        // Right Right Right
        assert!(btree2.root.children[2].children[1].children[2].leaf());
        assert_eq!(2, btree2.root.children[2].children[1].children[2].n());
        assert_eq!('y', btree2.root.children[2].children[1].children[2].keys[0]);
        assert_eq!('z', btree2.root.children[2].children[1].children[2].keys[1]);

        // Check BTree with Max. Degree 6

        // Root
        assert!(!btree3.root.leaf());
        assert_eq!(1, btree3.root.n());
        assert_eq!('i', btree3.root.keys[0]);

        // Left
        assert!(!btree3.root.children[0].leaf());
        assert_eq!(2, btree3.root.children[0].n());
        assert_eq!('c', btree3.root.children[0].keys[0]);
        assert_eq!('f', btree3.root.children[0].keys[1]);

        // Left Left
        assert!(btree3.root.children[0].children[0].leaf());
        assert_eq!(2, btree3.root.children[0].children[0].n());
        assert_eq!('a', btree3.root.children[0].children[0].keys[0]);
        assert_eq!('b', btree3.root.children[0].children[0].keys[1]);

        // Left Middle
        assert!(btree3.root.children[0].children[1].leaf());
        assert_eq!(2, btree3.root.children[0].children[1].n());
        assert_eq!('d', btree3.root.children[0].children[1].keys[0]);
        assert_eq!('e', btree3.root.children[0].children[1].keys[1]);

        // Left Right
        assert!(btree3.root.children[0].children[2].leaf());
        assert_eq!(2, btree3.root.children[0].children[2].n());
        assert_eq!('g', btree3.root.children[0].children[2].keys[0]);
        assert_eq!('h', btree3.root.children[0].children[2].keys[1]);

        // Right
        assert!(!btree3.root.children[1].leaf());
        assert_eq!(4, btree3.root.children[1].n());
        assert_eq!('l', btree3.root.children[1].keys[0]);
        assert_eq!('o', btree3.root.children[1].keys[1]);
        assert_eq!('r', btree3.root.children[1].keys[2]);
        assert_eq!('u', btree3.root.children[1].keys[3]);

        // Right Left
        assert!(btree3.root.children[1].children[0].leaf());
        assert_eq!(2, btree3.root.children[1].children[0].n());
        assert_eq!('j', btree3.root.children[1].children[0].keys[0]);
        assert_eq!('k', btree3.root.children[1].children[0].keys[1]);

        // Right Middle1
        assert!(btree3.root.children[1].children[1].leaf());
        assert_eq!(2, btree3.root.children[1].children[1].n());
        assert_eq!('m', btree3.root.children[1].children[1].keys[0]);
        assert_eq!('n', btree3.root.children[1].children[1].keys[1]);

        // Right Middle2
        assert!(btree3.root.children[1].children[2].leaf());
        assert_eq!(2, btree3.root.children[1].children[2].n());
        assert_eq!('p', btree3.root.children[1].children[2].keys[0]);
        assert_eq!('q', btree3.root.children[1].children[2].keys[1]);

        // Right Middle3
        assert!(btree3.root.children[1].children[3].leaf());
        assert_eq!(2, btree3.root.children[1].children[3].n());
        assert_eq!('s', btree3.root.children[1].children[3].keys[0]);
        assert_eq!('t', btree3.root.children[1].children[3].keys[1]);

        // Right Right
        assert!(btree3.root.children[1].children[4].leaf());
        assert_eq!(5, btree3.root.children[1].children[4].n());
        assert_eq!('v', btree3.root.children[1].children[4].keys[0]);
        assert_eq!('w', btree3.root.children[1].children[4].keys[1]);
        assert_eq!('x', btree3.root.children[1].children[4].keys[2]);
        assert_eq!('y', btree3.root.children[1].children[4].keys[3]);
        assert_eq!('z', btree3.root.children[1].children[4].keys[4]);
    }
}
