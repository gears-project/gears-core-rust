use actiondispatch::dispatchable::*;
use xfstruct::*;
use xfstate::XFState;
use flox;

pub struct Flox;

impl Flox {
    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        println!("Flox: {} - {}", node.id, state);
        match node.action.as_ref() {
            "evalexpr" => {
                println!("Flox: evalexpr {} - {}", node.id, state);
                match node.parameters {
                    Some(ref params) => {
                        match params.get("expression") {
                            Some(val) => {
                                error!("FLOX EXPRESSION {}", val);
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }
            _ => {
                println!("Flox: unimplemented/unhandled {} - {}", node.id, state);
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
