use structure::model::ModelDocument;
use structure::domain;

#[allow(dead_code)]
pub mod command_grammar {
    include!(concat!(env!("OUT_DIR"), "/command_grammar.rs"));
}

#[derive(Debug)]
pub enum DslItem {
    BlockOpen,
    BlockClose,
    With(String),
    Command(String),
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
    fn generate_dsl(&self) -> Vec<DslItem>;
    fn consume_dsl(&mut self, item: &Vec<DslItem>) -> Result<(), String>;
    fn interpret_dsl(&mut self, txt: &str) -> Result<(), String>;
}

pub fn dsl_out(items: &Vec<DslItem>) -> String {
    let indent_size = 4;
    let mut indent: usize = 0;
    let mut res = Vec::<String>::new();
    for item in items.iter() {
        match *item {
            DslItem::BlockOpen => {
                res.push(format!("{{"));
                indent += indent_size;
            }
            DslItem::BlockClose => {
                res.push(format!("}};"));
                indent -= indent_size;
            }
            DslItem::With(ref s) => {
                res.push(format!(" with {label}", label = s));
            }
            DslItem::Command(ref s) => {
                res.push(format!(" {cmd};", cmd = s));
            }
        }
    }

    res.join("\n")

}
