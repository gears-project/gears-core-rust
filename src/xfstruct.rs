use rustc_serialize::json;

use errors::XFlowError;

pub type XFlowEdge = (i32, i32);

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct XFlowStruct {
    pub id:           String,
    pub version:      i32,
    pub name:         String,
    pub requirements: Vec<XFlowRequirement>,
    pub variables:    XFlowVariables,
    pub nodes:        Vec<XFlowNode>,
    pub edges:        Vec<XFlowEdge>,
    pub branches:     Vec<XFlowBranch>,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct XFlowRequirement {
    pub xtype:   String,
    pub version: i32,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct XFlowVariableDefinition {
    pub name:  String,
    pub vtype: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct XFlowVariable {
    pub name:  String,
    pub vtype: String,
    pub value: String,
}

#[derive(RustcDecodable, RustcEncodable, Debug)]
pub struct XFlowVariables {
    pub input:  Vec<XFlowVariable>,
    pub local:  Vec<XFlowVariable>,
    pub output: Vec<XFlowVariableDefinition>,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Eq, PartialEq)]
pub struct XFlowNode {
    pub id:       i32,
    pub nodetype: String,
    pub label:    String,
    pub action:   String,
}

#[derive(RustcDecodable, RustcEncodable, Debug, Eq, PartialEq)]
pub struct XFlowBranch {
    pub name: String,
    pub edge: XFlowEdge,
}

impl XFlowStruct {
    /// Constructs a new `XFlowStruct`
    ///
    /// # Example
    /// ```
    /// use xflow::xfstruct::{XFlowStruct};
    /// let xfs = XFlowStruct::new();
    /// println!("XFlow version {}", xfs.id);
    /// ```
    pub fn new() -> XFlowStruct {
        XFlowStruct {
            id:          "".to_string(),
            name:        "".to_string(),
            version:      1,
            requirements: Vec::<XFlowRequirement>::new(),
            variables:    XFlowVariables {
                input:  Vec::<XFlowVariable>::new(),
                local:  Vec::<XFlowVariable>::new(),
                output: Vec::<XFlowVariableDefinition>::new(),
            },
            nodes:        Vec::<XFlowNode>::new(),
            edges:        Vec::<XFlowEdge>::new(),
            branches:     Vec::<XFlowBranch>::new(),
        }
    }

    /// Return a string representation of the XFlowStruct
    ///
    /// # Example
    /// ```
    /// use xflow::xfstruct::{XFlowStruct};
    /// let xfs = XFlowStruct::new();
    /// xfs.to_string();
    /// ```
    pub fn to_string(&self) -> String {
        format!("xflow {}", self.id)
    }

    /// Get `XFlowNode`s of `nodetype` and `action`
    ///
    /// # Example
    /// ```
    /// use xflow::xfstruct::{XFlowStruct};
    /// let xfs = XFlowStruct::new();
    /// let nodes = xfs.get_nodes_by("flow", "start");
    /// assert_eq!(nodes.len(), 0);
    /// ```
    pub fn get_nodes_by(&self, nodetype:&str, action:&str) -> Vec<&XFlowNode> {

        let res:Vec<&XFlowNode> = self.nodes.iter().filter({|node|
            node.nodetype == nodetype &&
                node.action == action
        }).collect();

        res
    }

    /// Get `XFlowNode`s of `nodetype`
    ///
    /// # Example
    /// ```
    /// use xflow::xfstruct::{XFlowStruct};
    /// let xfs = XFlowStruct::new();
    /// let nodes = xfs.get_nodes_of_type("flow");
    /// assert_eq!(nodes.len(), 0);
    /// ```
    pub fn get_nodes_of_type(&self, nodetype:&str) -> Vec<&XFlowNode> {

        let res:Vec<&XFlowNode> = self.nodes.iter().filter({|node|
            node.nodetype == nodetype
        }).collect();

        res
    }

    /// Return a JSON representation of the XFlowStruct
    ///
    /// # Example
    /// ```
    /// use xflow::xfstruct::{XFlowStruct};
    /// let xfs = XFlowStruct::new();
    /// xfs.to_json();
    /// ```
    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }

    /// Initialize a XFlowStruct from a JSON string
    ///
    /// # Example
    /// ```
    /// use xflow::xfstruct::{XFlowStruct};
    ///
    /// let empty_flow = "{\"id\":\"empty\",\"name\":\"empty\",\"version\":1,\"requirements\":[{\"xtype\":\"flow\",\"version\":1},{\"xtype\":\"flox\",\"version\":1}],\"variables\":{\"input\":[],\"output\":[],\"local\":[]},\"nodes\":[],\"edges\":[],\"branches\":[]}";
    ///
    /// let xfs = XFlowStruct::from_json(empty_flow);
    /// println!("XFlow has version {}", xfs.version);
    /// ```
    pub fn from_json(json_string:&str) -> XFlowStruct {
        let xfs:XFlowStruct = json::decode(json_string).unwrap();
        xfs
    }

    pub fn get_in_edges(&self, node:&XFlowNode) -> Vec<&XFlowEdge> {

        let res:Vec<&XFlowEdge> = self.edges.iter().filter({|edge|
            edge.1 == node.id
        }).collect();

        res
    }

    pub fn get_out_edges(&self, node:&XFlowNode) -> Vec<&XFlowEdge> {

        let res:Vec<&XFlowEdge> = self.edges.iter().filter({|edge|
            edge.0 == node.id
        }).collect();

        res
    }

    pub fn get_branches_for(&self, edge:&XFlowEdge) -> Vec<&XFlowBranch> {

        let res:Vec<&XFlowBranch> = self.branches.iter().filter({|branch|
            edge.0 == branch.edge.0 &&
                edge.1 == branch.edge.1
        }).collect();

        res

    }

    pub fn get_entry_node(&self) -> Result<&XFlowNode, XFlowError> {
        let res = self.get_nodes_by("flow", "start");
        match res.len() {
            0 => Err(XFlowError::NoEntryNode),
            1 => Ok(res[0]),
            _ => Err(XFlowError::MultipleEntryNodes),
        }
    }

    pub fn get_terminal_nodes(&self) -> Result<Vec<&XFlowNode>, XFlowError> {
        let res = self.get_nodes_by("flow", "end");
        match res.len() {
            0 => Err(XFlowError::NoTerminalNode),
            _ => Ok(res),
        }
    }

}

