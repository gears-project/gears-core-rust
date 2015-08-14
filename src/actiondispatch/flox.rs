use ::xflow::xfstruct::*;
use ::actiondispatch::dispatchable::*;

pub struct Flox {
    ready: bool
}

impl Flox {

    pub fn new() -> Flox {
        Flox {
            ready: false
        }
    }

    fn process_node(&self, node:&XFlowNode) -> () {
    }

}

impl Dispatchable for Flox {
    fn init(&self) -> Result<String, String> {
        Ok("ok".to_string())

    }

    fn dispatch(&self, node:&XFlowNode, state:String) -> Result<String, String> {
        self.process_node(node);
        Ok("ok".to_string())
    }

}


