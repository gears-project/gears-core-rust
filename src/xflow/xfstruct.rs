use rustc_serialize::json;

pub type XFlowEdge = [i32; 2];

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations

#[derive(RustcDecodable, RustcEncodable)]
pub struct XFlowStruct {
    pub id:       String,
    pub version:  i32,
    pub name:     String,
    pub nodes:    Vec<XFlowNode>,
    pub edges:    Vec<XFlowEdge>,
    pub branches: Vec<XFlowBranch>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct XFlowRequirement {
    pub xtype:   String,
    pub version: i32,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct XFlowVariable {
    pub name:  String,
    pub vtype: String,
    pub value: String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct XFlowVariables {
    pub input:  Vec<XFlowVariable>,
    pub output: Vec<XFlowVariable>,
    pub local:  Vec<XFlowVariable>,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct XFlowNode {
    pub id:       i32,
    pub nodetype: String,
    pub label:    String,
    pub action:   String,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct XFlowBranch {
    pub name: String,
    pub edge: XFlowEdge,
}

impl XFlowStruct {
    /// Constructs a new `XFlowStruct`
    ///
    /// # Example
    /// ```
    /// use xfdocs::xflow::xfstruct::{XFlowStruct};
    /// let xfs = XFlowStruct::new();
    /// ```
    pub fn new() -> XFlowStruct {
        XFlowStruct {
            id:       "".to_string(),
            name:     "".to_string(),
            version:  1,
            nodes:    Vec::<XFlowNode>::new(),
            edges:    Vec::<XFlowEdge>::new(),
            branches: Vec::<XFlowBranch>::new(),
        }
    }

    pub fn to_string(&self) -> String {
        format!("xflow {}", self.id)
    }

    pub fn get_entry_nodes(&self) -> usize {

        let mut counter:usize = 0;

        for node in self.nodes.iter() {

            if node.nodetype == "flow" &&
                node.action == "start" {
                    counter = counter + 1;
                }
        }

        counter

    }

    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }

    pub fn from_json(json_string:String) -> XFlowStruct {
        let xfs:XFlowStruct = json::decode(&json_string).unwrap();
        xfs
    }

}

