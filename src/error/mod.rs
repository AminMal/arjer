#[derive(Debug)]
pub enum ParseError {
    EOF,
    UnexpectedToken { expected: Vec<String>, got: String },
    InvalidNumber { tpe: String, value: String },
    InvalidJsonStructure,
}
