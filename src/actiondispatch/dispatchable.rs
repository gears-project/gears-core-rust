use xfstruct::*;
use xfstate::XFState;

pub trait Dispatchable {
    fn init(&mut self) -> Result<String, String>;

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<String, String>;
}
