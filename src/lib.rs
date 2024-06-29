pub mod btree;

#[cfg(test)]
mod tests {

    use crate::btree::{BTree};

    #[test]
    fn construct_empty_btree() {
        let _btree = BTree::<i32>::new(1);
    }
}
