use std;
use std::fmt;


#[derive(Debug, Clone)]
pub struct Error {
    pub message: String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}


#[test]
fn test_create() {
    let error = Error{message: "something is wrong".to_string()};
    println!("{:?}", error);
    assert_eq!(error.message, "something is wrong");
}

#[test]
fn test_display() {
    let error = Error{message: "something is wrong".to_string()};
    println!("{}", error);
    assert_eq!(error.message, "something is wrong");
}
