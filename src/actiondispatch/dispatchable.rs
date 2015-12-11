use xfstruct::*;

pub trait Dispatchable {
    fn init(&mut self) -> Result<String, String>;

    fn dispatch(&self, node: &XFlowNode, state: &str) -> Result<String, String>;

}
