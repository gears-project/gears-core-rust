use structure::model::ModelDocument;
use structure::domain;

#[allow(dead_code)]
pub mod command_grammar {
    include!(concat!(env!("OUT_DIR"), "/command_grammar.rs"));
}

#[derive(Debug)]
pub enum DslToken {
    BlockOpen,
    BlockClose,
    With(String),
    Command(String),
    Comment(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum XFlowCommand {
    AddNode(String),
}

#[derive(Debug, Eq, PartialEq)]
pub enum ConfigCommand {
    SetDefaultLocale(String),
}

pub trait GearsDsl {
    fn generate_dsl(&self) -> Vec<DslToken>;
    fn consume_dsl(&mut self, item: &Vec<DslToken>) -> Result<(), String>;

    fn interpret_dsl(&mut self, txt: &str) -> Result<(), String> {
        match command_grammar::expression(&txt) {
            Ok(dsl_items) => {
                match self.consume_dsl(&dsl_items) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        error!("interpret_dsl : error with commands : {:?}", err);
                        return Err(format!("interpret_dsl : error with commands : {:?}", err));
                    }
                }
            }
            Err(err) => {
                error!("interpret_dsl : error : {:?}", err);
                return Err(format!("interpret_dsl : error : {:?}", err));
            }
        }
    }

    fn to_text_dsl(&self) -> String {
        let items = &self.generate_dsl();
        let indent_size = 4;
        let mut indent: usize = 0;
        let mut res = Vec::<String>::new();
        for item in items.iter() {
            match *item {
                DslToken::BlockOpen => {
                    res.push(format!("{{"));
                    indent += indent_size;
                }
                DslToken::BlockClose => {
                    res.push(format!("}};"));
                    indent -= indent_size;
                }
                DslToken::With(ref s) => {
                    res.push(format!(" with {label}", label = s));
                }
                DslToken::Command(ref s) => {
                    res.push(format!(" {cmd};", cmd = s));
                }
                DslToken::Comment(_) => {}
            }
        }

        res.join("\n")

    }
}
