use std::collections::HashMap;
use xfstruct::XFlowVariable;

type XFStore = HashMap<String, XFlowVariable>;

pub struct XFState {
    store: XFStore,
}

fn copy_xvar(xvar: &XFlowVariable) -> XFlowVariable {
    XFlowVariable {
        name: xvar.name.clone(),
        vtype: xvar.vtype.clone(),
        value: xvar.value.clone(),
    }
}

impl XFState {
    /// Constructs a new `XFState`
    ///
    /// # Example
    /// ```
    /// use xflow::xfstate::{XFState};
    /// let xfstate = XFState::new();
    /// println!("State has {} keys", xfstate.len());
    /// ```
    pub fn new() -> XFState {
        let store: XFStore = HashMap::new();

        XFState { store: store }
    }

    pub fn len(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.store.is_empty()
    }

    pub fn get(&self, name: &str) -> Option<&XFlowVariable> {
        self.store.get(name)
    }

    pub fn has(&self, name: &str) -> bool {
        self.store.contains_key(name)
    }

    pub fn add(&mut self, xvar: &XFlowVariable) {
        self.store.insert(xvar.name.clone(), copy_xvar(xvar));
    }
}
