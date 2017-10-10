use structure::xflow::*;
use runtime::xfstate::XFState;

pub trait Dispatchable {
    fn init(&mut self) -> Result<(), ()>;

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<(), ()>;
}
