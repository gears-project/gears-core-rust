use ::xflow::xfstruct::*;

pub trait Dispatchable {
    fn init(&self) -> Result<String, String>;

    fn dispatch(&self, node:&XFlowNode, state:String) -> Result<String, String>;

}


