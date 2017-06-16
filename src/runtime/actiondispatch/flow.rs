use runtime::actiondispatch::dispatchable::*;
use structure::xflow::*;
use xfstate::XFState;

pub struct Flow;

impl Flow {
    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        debug!("Flow: {} - {}", node.id, state);
        match node.action.as_ref() {
            "start" => {
                info!("Start {} - {}", node.id, state);
            }
            "end" => {
                info!("End {} - {}", node.id, state);
            }
            "branch" => {
                info!("Branch {} - {}", node.id, state);
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
