use actiondispatch::dispatchable::*;
use xfstruct::*;
use xfstate::XFState;

pub struct Flow {
    ready: bool,
}

impl Flow {
    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        println!("Flow: {} - {}", node.id, state);
        match node.action.as_ref() {
            "start" => {
                println!("Flow: start {} - {}", node.id, state);
            }
            "end" => {
                println!("Flow: end {} - {}", node.id, state);
            }
            "branch" => {
                println!("Flow: branch {} - {}", node.id, state);
            }
            _ => {
                println!("Flow: unimplemented/unhandled {} - {}", node.id, state);
            }

        }
    }
}

impl Default for Flow {
    fn default() -> Self {
        Flow { ready: false }
    }
}

impl Dispatchable for Flow {
    fn init(&mut self) -> Result<String, String> {
        self.ready = true;
        Ok("ok".to_owned())
    }

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<String, String> {
        self.process_node(node, state);
        Ok("ok".to_owned())
    }
}
