use xfstruct::XFlowValue;

#[allow(dead_code)]
mod flox_grammar {
    include!(concat!(env!("OUT_DIR"), "/flox_grammar.rs"));
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    ParseError(String),
}

pub fn parse(input: &str) -> Result<XFlowValue, Error> {
    match flox_grammar::arithmetic_expression(input) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }
    }
}

pub fn parse_arithmetic(input: &str) -> Result<XFlowValue, Error> {
    match flox_grammar::arithmetic_expression(input) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }
    }
}

pub fn parse_boolean(input: &str) -> Result<XFlowValue, Error> {
    match flox_grammar::boolean_expression(input) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }
    }
}
