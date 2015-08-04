use rustc_serialize::json;

pub type XFlowEdge = (i32, i32);

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
    pub fn new() -> XFlowStruct {
        create_xflow_struct()
    }

    pub fn to_string(&self) -> String {
        format!("xflow {}", self.id)
    }

    pub fn get_entry_nodes(&self) -> usize {

        let iterator = &self.nodes.iter();
        let filtered = iterator.clone().filter({|&node|
            node.nodetype == "flow" &&
                node.action == "start"
        });

        let num = filtered.count();

        if num > 1 {
            panic!("More than one entry node found!");
        } else if num < 1 {
            panic!("No entry nodes found!");
        }

        num

    }

    pub fn to_json(&self) -> String {
        json::encode(&self).unwrap()
    }

    pub fn from_json(json_string:String) -> XFlowStruct {
        let xfs:XFlowStruct = json::decode(&json_string).unwrap();
        xfs
    }

}

//
// Test code
//

fn x_get_rand() -> i32 {
    5 // rand::thread_rng().gen_range(1, 11)
}

fn create_node(id:i32) -> XFlowNode {
    XFlowNode {
        id: id,
        nodetype: "flow".to_string(),
        action: "some action".to_string(),
        label: "some name".to_string()
    }
}

fn create_nodes(amount:i32) -> Vec<XFlowNode> {
    let mut nodes = Vec::<XFlowNode>::new();

    for i in 0..amount {
        nodes.push(create_node(i))
    }

    return nodes
}

fn create_edges(amount:i32) -> Vec<(i32, i32)> {
    let left   = x_get_rand();
    let right  = x_get_rand();

    let mut edges = Vec::<XFlowEdge>::new();

    for i in 0..amount {
        edges.push((left, right + i));
    }

    edges
}

fn create_branches(amount:i32) -> Vec<XFlowBranch> {

    let mut branches = Vec::<XFlowBranch>::new();

    for i in 0..amount {
        let left   = x_get_rand();
        let right  = x_get_rand();
        branches.push(
            XFlowBranch {
                name: "Some branch".to_string(),
                edge : (left, right + i)
            })
    }

    branches
}

fn create_xflow_struct() -> XFlowStruct {
    XFlowStruct {
        id:       "id1".to_string(),
        version:  1,
        name:     "Some name".to_string(),
        nodes:    create_nodes(5),
        edges:    create_edges(5),
        branches: create_branches(5)
    }
}

