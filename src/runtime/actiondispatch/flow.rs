use crate::runtime::actiondispatch::dispatchable::*;
use crate::structure::xflow::*;
use crate::runtime::xfstate::XFState;

#[derive(Debug)]
pub struct Flow;

impl Flow {
    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        debug!("Flow: {} - {}", node.id, state);
        match node.action.as_ref() {
            "start" => {
                debug!("Start {} - {}", node.id, state);
            }
            "end" => {
                debug!("End {} - {}", node.id, state);
            }
            "branch" => {
                debug!("Branch {} - {}", node.id, state);
            }
            _ => {
                error!("Unimplemented/unhandled {} - {}", node.id, state);
            }

        }
    }
}

impl Default for Flow {
    fn default() -> Self {
        Flow {}
    }
}

impl Dispatchable for Flow {
    fn init(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<(), ()> {
        self.process_node(node, state);
        Ok(())
    }
}
