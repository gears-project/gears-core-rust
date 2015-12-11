use actiondispatch::dispatchable::*;
use xfstruct::*;

pub struct Flox {
    ready: bool,
}

impl Flox {
    pub fn new() -> Flox {
        Flox { ready: false }
    }

    fn process_node(&self, node: &XFlowNode, state: &str) -> () {
        println!("Flox: {} - {}", node.id, state);
    }
}

impl Dispatchable for Flox {
    fn init(&mut self) -> Result<String, String> {
        self.ready = true;
        Ok("ok".to_owned())

    }

    fn dispatch(&self, node: &XFlowNode, state: &str) -> Result<String, String> {
        self.process_node(node, state);
        Ok("ok".to_owned())
    }
}
