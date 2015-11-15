use std::collections::HashMap;
use xfstruct::XFlowVariable;

type XFStore = HashMap<String, XFlowVariable>;

pub struct XFState<'a> {
    store: &'a XFStore
}

fn copyXvar(xvar:&XFlowVariable) -> XFlowVariable {
    XFlowVariable {
        name:  xvar.name.to_string(),
        vtype: xvar.vtype.to_string(),
        value: xvar.value.to_string()
    }
}

impl XFState {

    pub fn new() -> XFState {
        let mut store: XFStore = HashMap::new();

        XFState {
            store: store
        }
    }

    pub fn get(&self, name:&str) -> Option<&XFlowVariable> {
        self.store.get(name)
    }

    pub fn has(&self, name:&str) -> bool {
        self.store.contains_key(name)
    }

    pub fn add(&self, xvar:&XFlowVariable) {
        self.store.insert(xvar.name.to_string(), copyXvar(xvar));
    }


}

