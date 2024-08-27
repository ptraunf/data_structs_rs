use crate::Result;
pub trait Set {
    type T;
    fn insert(&mut self, e: Self::T);
    fn contains(&self, e: Self::T) -> bool;
    fn equals(&self, other: Self) -> bool;
    fn union(&self, other: Self) -> Result<Self> where Self: Sized;
    fn intersection(&self, other: Self) -> Result<Self> where Self: Sized;
    fn difference(&self, other: Self) -> Result<Self> where Self: Sized;
}


