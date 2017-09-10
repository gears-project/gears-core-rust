#[allow(dead_code)]
pub mod command_grammar {
    include!(concat!(env!("OUT_DIR"), "/command_grammar.rs"));
}

#[derive(Debug, PartialEq, Eq)]
pub enum DslToken {
    BlockOpen,
    BlockClose,
    With(String),
    Command(String),
    Comment(String),
}

pub type DslTokens = Vec<DslToken>;

#[derive(Debug)]
pub enum DslTree {
    Scope(String, Vec<DslTree>),
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
    fn generate_dsl(&self) -> DslTokens;
    fn consume_command(&mut self, cmd: &str) -> Result<(), String>;
    fn consume_scope(&mut self, s: &str, tree: &Vec<DslTree>) -> Result<(), String>;

    fn consume_dsl_tree(&mut self, items: &Vec<DslTree>) -> Result<(), String> {
        debug!("consume_dsl_tree : items : '{:?}'", items);
        // XXX: Item can error out and still return Ok
        for item in items {
            match *item {
                DslTree::Scope(ref s, ref tree) => {
                    debug!("consume_dsl_tree : matching scope item '{:?}'", s);
                    self.consume_scope(s, tree);
                }
                DslTree::Command(ref s) => {
                    debug!("consume_dsl_tree command '{:?}'", s);
                    match self.consume_command(&s) {
                        Err(err) => {
                            return Err(err);
                        }
                        _ => {}
                    }
                }
                DslTree::Comment(ref s) => {
                    debug!("consume_dsl_tree comment '{:?}'", s);
                }
            }
        }
        Ok(())
    }

    fn consume_dsl(&mut self, items: &[DslToken]) -> Result<(), String> {
        match tokens_as_tree(&items) {
            Ok(tree) => self.consume_dsl_tree(&tree),
            Err(err) => Err(err),
        }
    }

    fn generate_dsl_tree(&self) -> Result<Vec<DslTree>, String> {
        tokens_as_tree(&self.generate_dsl())
    }

    fn interpret_dsl(&mut self, txt: &str) -> Result<(), String> {
        debug!("interpret_dsl : '{:?}'", txt);
        match command_grammar::expression(&txt) {
            Ok(dsl_items) => {
                match tokens_as_tree(&dsl_items) {
                    Ok(tree) => self.consume_dsl_tree(&tree),
                    Err(err) => {
                        error!("interpret_dsl : error with commands : {:?}", err);
                        Err(format!("interpret_dsl : error with commands : {:?}", err))
                    }
                }
            }
            Err(err) => {
                error!("interpret_dsl : error : {:?}", err);
                Err(format!("interpret_dsl : error : {:?}", err))
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
                    res.push(format!("{{\n"));
                    indent += indent_size;
                }
                DslToken::BlockClose => {
                    res.push(format!("}};\n"));
                    indent -= indent_size;
                }
                DslToken::With(ref s) => {
                    res.push(format!(" with {label} ", label = s));
                }
                DslToken::Command(ref s) => {
                    res.push(format!("  {cmd};\n", cmd = s));
                }
                DslToken::Comment(_) => {}
            }
        }

        res.join("")
    }
}

pub fn tokens_as_tree(tokens: &[DslToken]) -> Result<Vec<DslTree>, String> {

    fn to_tree(tokens: &[DslToken], offset: &mut usize) -> Result<Vec<DslTree>, String> {
        let mut res = Vec::<DslTree>::new();

        let mut subject = "".to_owned();

        debug!("tokens_as_tree entry {} : {}", offset, *offset);

        while *offset < tokens.len() {
            // debug!("tokens_as_tree loop {} : {}", offset, *offset);
            match tokens[*offset] {
                DslToken::BlockOpen => {
                    *offset += 1;
                    match to_tree(&tokens, offset) {
                        Ok(out) => {
                            res.push(DslTree::Scope(subject.clone(), out));
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
                DslToken::BlockClose => {
                    return Ok(res);
                }
                DslToken::With(ref s) => {
                    subject = s.clone();

                    // Lookahead
                    if *offset >= (tokens.len() - 1) {
                        return Err(
                            "tokens_as_tree : With statement ends prematurely".to_owned(),
                        );
                    } else if (tokens[*offset + 1]).ne(&DslToken::BlockOpen) {
                        return Err(
                            "tokens_as_tree : Encountered With statement not followed by BlockOpen"
                                .to_owned(),
                        );
                    }
                }
                DslToken::Comment(ref c) => {
                    res.push(DslTree::Comment((*c).clone()));
                }
                DslToken::Command(ref c) => {
                    res.push(DslTree::Command((*c).clone()));
                }
            }
            *offset += 1;
        }

        Ok(res)
    }

    let mut offset: usize = 0;
    to_tree(&tokens, &mut offset)
}
