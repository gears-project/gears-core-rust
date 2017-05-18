use actiondispatch::dispatchable::Dispatchable;
use std::collections::HashMap;
use xfstruct::XFlowNode;
use xfstate::XFState;

type Receivers<'a> = HashMap<String, Box<Dispatchable + 'a>>;

pub struct Dispatcher<'a> {
    receivers: Receivers<'a>,
}

impl<'a> Dispatcher<'a> {
    pub fn register_receiver<T: Dispatchable + 'a>(&mut self, name: &str, receiver: T) -> () {
        let receiver_box = Box::new(receiver);
        self.receivers.insert(name.to_owned(), receiver_box);
    }

    pub fn dispatch(&self, xfnode: &XFlowNode, xfstate: &mut XFState) -> Result<(), ()> {
        info!("Nodetype {}, action {}", xfnode.nodetype, xfnode.action);

        if let Some(receiver) = self.receivers.get(&xfnode.nodetype) {
            receiver.dispatch(xfnode, xfstate);
            Ok(())
        } else {
            error!("No dispatcher found for {}/{}!",
                   xfnode.nodetype,
                   xfnode.action);
            Err(())
        }

    }
}

impl<'a> Default for Dispatcher<'a> {
    fn default() -> Dispatcher<'a> {
        let receivers: Receivers = HashMap::new();

        Dispatcher { receivers: receivers }
    }
}
