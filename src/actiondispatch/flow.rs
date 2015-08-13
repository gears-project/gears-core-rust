use ::xflow::xfstruct::*;

trait Dispatchable {
    fn init(&self) -> Result<String, String>;

    fn dispatch(&self, node:&XFlowNode, state:String) -> Result<String, String>;

}

pub struct Flow {
    ready: bool
}

impl Flow {

    pub fn new() -> Flow {
        Flow {
            ready: false
        }
    }

    fn process_node(&self, node:&XFlowNode) -> () {
    }

}

impl Dispatchable for Flow {
    fn init(&self) -> Result<String, String> {
        Ok("ok".to_string())

    }

    fn dispatch(&self, node:&XFlowNode, state:String) -> Result<String, String> {
        self.process_node(node);
        Ok("ok".to_string())
    }

}

