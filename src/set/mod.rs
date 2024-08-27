pub trait Set {
    type T;
    fn insert(&mut self, e: Self::T);
    fn contains(&self, e: Self::T) -> bool;
}


