pub mod cpu;
pub mod instruction;

#[derive(Debug, PartialEq, Eq)]
pub enum TenthError {
    FileProblem(String),
    InputInvalid(String),
}

impl From<nom::error::Error<&str>> for TenthError {
    fn from(e: nom::error::Error<&str>) -> Self {
        Self::InputInvalid(format!("{}", e))
    }
}

impl From<std::io::Error> for TenthError {
    fn from(e: std::io::Error) -> Self {
        Self::FileProblem(format!("{}", e))
    }
}
