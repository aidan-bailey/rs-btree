#[derive(Clone)]
pub struct Record<T> {
    key: usize,
    value: T,
}

impl<T> Record<T> {
    pub fn new(key: usize, value: T) -> Self {
        Record { key, value }
    }

    pub fn key(&self) -> &usize {
        &self.key
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}
