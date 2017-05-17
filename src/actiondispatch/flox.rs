use actiondispatch::dispatchable::*;
use xfstruct::*;
use xfstate::XFState;
use flox;

pub struct Flox;

impl Flox {
    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        debug!("Flox: {} - {}", node.id, state);
        match node.action.as_ref() {
            "evalexpr" => {
                info!("Flox: evalexpr {} - {}", node.id, state);
                match node.parameters {
                    Some(ref params) => {
                        match params.get("expression") {
                            Some(val) => {
                                debug!("Expression : {}", val);
                                match flox::parse(val.as_str().unwrap()) {
                                    Ok(res) => {
                                        debug!("Expression {} - Result - {:?}", val, res);
                                    }
                                    Err(err) => {
                                        error!("Expression {} - Result - {:?}", val, err);
                                    }
                                };
                            }
                            None => {
                                error!("No expression found in parameters");
                            }
                        }
                    }
                    None => {
                        error!("No parameters found in node");
                    }
                }
            }
            _ => {
                error!("Unimplemented/unhandled action : {} - {}", node.id, state);
            }

        }
    }
}

impl Default for Flox {
    fn default() -> Self {
        Flox {}
    }
}

impl Dispatchable for Flox {
    fn init(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<(), ()> {
        self.process_node(node, state);
        Ok(())
    }
}
