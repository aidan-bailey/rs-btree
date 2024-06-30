use super::record::Record;

pub trait Node<T: Clone> {
    fn find(&self, key: &usize) -> Result<Option<&Record<T>>, &'static str>;
}
