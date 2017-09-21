use std::collections::HashMap;

use runtime::actiondispatch::dispatchable::Dispatchable;
use structure::xflow::{XFlowNode, XFlowNodeType};
use xfstate::XFState;

type Receivers<'a> = HashMap<XFlowNodeType, Box<Dispatchable + 'a>>;

pub struct Dispatcher<'a> {
    receivers: Receivers<'a>,
}

impl<'a> Dispatcher<'a> {
    pub fn register_receiver<T: Dispatchable + 'a>(
        &mut self,
        xflow_node_type: XFlowNodeType,
        receiver: T,
    ) -> () {
        let receiver_box = Box::new(receiver);
        self.receivers.insert(
            xflow_node_type.to_owned(),
            receiver_box,
        );
    }

    pub fn dispatch(&self, xfnode: &XFlowNode, xfstate: &mut XFState) -> Result<(), String> {
        debug!("Nodetype {:?}, action {}", xfnode.nodetype, xfnode.action);

        if let Some(receiver) = self.receivers.get(&xfnode.nodetype) {
            match receiver.dispatch(xfnode, xfstate) {
                Ok(()) => Ok(()),
                Err(()) => {
                    let msg = "An error has occurred dispatching, but this is currently not handled".to_owned();
                    error!("{}", msg);
                    Err(msg)
                }
            }
        } else {
            let msg = format!(
                "No dispatcher found for {:?}/{}!",
                xfnode.nodetype,
                xfnode.action
            );
            error!("{}", msg);
            Err(msg)
        }

    }
}

impl<'a> Default for Dispatcher<'a> {
    fn default() -> Dispatcher<'a> {
        let receivers: Receivers = HashMap::new();

        Dispatcher { receivers: receivers }
    }
}
