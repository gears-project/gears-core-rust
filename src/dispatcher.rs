use actiondispatch::dispatchable::Dispatchable;
use std::collections::HashMap;
use xfstruct::XFlowNode;
use xfstate::XFState;

type Dispatchers<'a> = HashMap<String, Box<Dispatchable + 'a>>;

pub struct Dispatcher<'a> {
    dispatchers: Dispatchers<'a>,
}

impl<'a> Dispatcher<'a> {
    pub fn new() -> Dispatcher<'a> {
        let dispatchers: Dispatchers = HashMap::new();

        Dispatcher { dispatchers: dispatchers }
    }

    pub fn register_dispatcher<T: Dispatchable + 'a>(&mut self, name: &str, dispatcher: T) -> () {
        let disp_box = Box::new(dispatcher);
        self.dispatchers.insert(name.to_owned(), disp_box);
    }

    pub fn dispatch(&self, xfnode: &XFlowNode, xfstate: &mut XFState) -> bool {
        println!("Dispatch {}/{}!", xfnode.nodetype, xfnode.action);

        if let Some(dispatch) = self.dispatchers.get(&xfnode.nodetype) {
            dispatch.dispatch(xfnode, xfstate);
            true
        } else {
            false
        }

    }
}
