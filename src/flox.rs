#[allow(dead_code)]
mod flox_grammar {
    include!(concat!(env!("OUT_DIR"), "/flox_grammar.rs"));
}

#[derive(Debug)]
pub enum FloxResult {
    String(String),
    Integer(i64),
    Boolean(bool),
}

#[derive(Debug)]
pub enum Error {
    ParseError(String),
}

pub fn parse(input: &str) -> Result<FloxResult, Error> {
    match flox_grammar::expression(input) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }

    }
}

pub fn parse_boolean(input: &str) -> Result<FloxResult, Error> {
    match flox_grammar::boolean_expression(input) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }

    }
}
