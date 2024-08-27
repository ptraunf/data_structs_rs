use std::fmt::Formatter;

pub mod set;
pub mod bloom_filter;
pub mod queue;

#[derive(Debug)]
pub enum Error {
    IllegalArguments(String),


}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}
pub type Result<T> = core::result::Result<T, Error>;
