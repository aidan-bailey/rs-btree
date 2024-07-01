#[derive(Clone, Debug)]
pub struct Record<KT: Ord + Copy, DT> {
    key: KT,
    value: DT,
}

impl<KT: Ord + Copy, DT> Record<KT, DT> {
    pub fn new(key: KT, value: DT) -> Self {
        Record { key, value }
    }

    pub fn key(&self) -> KT {
        self.key
    }

    pub fn value(&self) -> &DT {
        &self.value
    }
}
