use actiondispatch::dispatchable::*;
use xfstruct::*;
use xfstate::XFState;

pub struct Flox {
    ready: bool,
}

impl Flox {
    pub fn new() -> Flox {
        Flox { ready: false }
    }

    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        println!("Flox: {} - {}", node.id, state);
        match node.action.as_ref() {
            "evalexpr" => {
                println!("Flox: evalexpr {} - {}", node.id, state);
            }
            _ => {
                println!("Flox: unimplemented/unhandled {} - {}", node.id, state);
            }

        }
    }
}

impl Dispatchable for Flox {
    fn init(&mut self) -> Result<String, String> {
        self.ready = true;
        Ok("ok".to_owned())

    }

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<String, String> {
        self.process_node(node, state);
        Ok("ok".to_owned())
    }
}
