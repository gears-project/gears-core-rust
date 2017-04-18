#[allow(dead_code)]
mod flox_grammar {
    include!(concat!(env!("OUT_DIR"), "/flox_grammar.rs"));
}

#[derive(Debug)]
pub enum FloxOutput {
    String(String),
    Integer(i64),
    Boolean(bool),
}

#[derive(Debug)]
pub enum FloxError {
    ParseError(String),
}

pub fn parse(input: &str) -> Result<FloxOutput, FloxError> {
    Err(FloxError::ParseError("Unimplemented!".into()))
}
