use actiondispatch::dispatchable::*;
use xfstruct::*;

pub struct Flow {
    ready: bool,
}

impl Flow {
    pub fn new() -> Flow {
        Flow { ready: false }
    }

    fn process_node(&self, node: &XFlowNode, state: &str) -> () {
        println!("Flow: {} - {}", node.id, state);
    }
}

impl Dispatchable for Flow {
    fn init(&mut self) -> Result<String, String> {
        self.ready = true;
        Ok("ok".to_owned())

    }

    fn dispatch(&self, node: &XFlowNode, state: &str) -> Result<String, String> {
        self.process_node(node, state);
        Ok("ok".to_owned())
    }
}
