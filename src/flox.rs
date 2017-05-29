use xfstruct::XFlowValue;
use xfstate::XFState;

#[allow(dead_code)]
mod flox_grammar {
    include!(concat!(env!("OUT_DIR"), "/flox_grammar.rs"));
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Error {
    ParseError(String),
}

pub fn parse_context(input: &str, state: &XFState) -> Result<XFlowValue, Error> {
    match flox_grammar::expression(input, state) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }
    }
}

pub fn parse(input: &str) -> Result<XFlowValue, Error> {
    let state = XFState::default();
    match flox_grammar::expression(input, &state) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }
    }
}

pub fn parse_arithmetic(input: &str) -> Result<XFlowValue, Error> {
    let state = XFState::default();
    match flox_grammar::arithmetic_expression(input, &state) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }
    }
}

pub fn parse_boolean(input: &str) -> Result<XFlowValue, Error> {
    let state = XFState::default();
    match flox_grammar::boolean_expression(input, &state) {
        Ok(res) => Ok(res),
        Err(err) => {
            Err(Error::ParseError(format!("Bad expression {:?} - Error : {:?}", input, err)))
        }
    }
}
