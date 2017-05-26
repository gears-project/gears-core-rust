use std::collections::HashMap;
use std::fmt;
use xfstruct::XFlowVariable;

pub type XFStore = HashMap<String, XFlowVariable>;

#[derive(Serialize, Deserialize, Debug)]
pub struct XFState {
    pub store: XFStore,
}

fn copy_xvar(xvar: &XFlowVariable) -> XFlowVariable {
    XFlowVariable {
        name: xvar.name.clone(),
        vtype: xvar.vtype.clone(),
        value: xvar.value.clone(),
    }
}

impl XFState {
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
        self.store.insert(xvar.name.clone(), copy_xvar(xvar)); // (*xvar).clone());
    }

    pub fn remove(&mut self, name: &str) {
        self.store.remove(name);
    }
}

impl Default for XFState {
    /// Constructs a new `XFState`
    ///
    /// # Example
    /// ```
    /// use xflow::xfstate::{XFState};
    /// let xfstate = XFState::default();
    /// println!("State has {} keys", xfstate.len());
    /// ```
    fn default() -> Self {
        let store: XFStore = HashMap::new();

        XFState { store: store }
    }
}

impl fmt::Display for XFState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.store.keys())
    }
}
