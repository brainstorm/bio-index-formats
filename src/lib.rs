//#[macro_use]
extern crate nom;
pub mod parser_bai;
pub mod csi;

#[derive(Default)]
pub struct ParseError;
impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "A parsing error occurred.")
    }
}
impl std::fmt::Debug for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <ParseError as std::fmt::Display>::fmt(self, f)
    }
}
impl std::error::Error for ParseError { }