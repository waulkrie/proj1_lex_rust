use std::fmt;
use std::fmt::Display;

#[derive(Debug)]
pub struct SyntaxError {
    pub(crate) message: String,
    pub(crate) line_no: i32,
    pub(crate) function: String,
}

// declare method
impl std::error::Error for SyntaxError {}

// define method
impl Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SYNTAX ERROR in {} at Line:{} {}", self.function, self.line_no, self.message)
    }
}
