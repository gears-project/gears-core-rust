use structure::xflow::*;
use xfstate::XFState;

pub trait Dispatchable {
    fn init(&mut self) -> Result<(), ()>;

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<(), ()>;
}
