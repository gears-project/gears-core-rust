extern crate rand;

use rand::Rng;

struct XFlowStruct {
    id:       i32,
    version:  i32,
    name:     String,
    nodes:    Vec<XFlowNode>,
    edges:    Vec<(i32, i32)>,
    branches: Vec<XFlowBranch>,
}

struct XFlowRequirement {
    xtype: String,
    version: i32
}

struct XFlowVariable {
    name:  String,
    vtype: String,
    value: String
}

struct XFlowVariables {
    input:  Vec<XFlowVariable>,
    output: Vec<XFlowVariable>,
    local:  Vec<XFlowVariable>,
}

struct XFlowNode {
    id:       i32,
    nodetype: String,
    label:    String,
    action:   String,
}

struct XFlowBranch {
    name: String,
    edge: (i32, i32)
}

impl XFlowStruct {
    fn new() -> XFlowStruct {
        create_xflow_struct()
    }

    fn to_string(&self) -> String {
        format!("xflow {}", self.id)
    }

    fn get_entry_node(&self) -> XFlowNode {
        let res = self.nodes.filter({|&node|
            node.label == "label"
        });
        res.0
    }

}

//
// Test code
//

fn get_rand() -> i32 {
    rand::thread_rng().gen_range(1, 11)
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

    let i:i32 = 0;

    for i in 0..amount {
        nodes.push(create_node(i))
    }

    return nodes
}

fn create_edges(amount:i32) -> Vec<(i32, i32)> {
    let left   = get_rand();
    let right  = get_rand();

    let mut edges = Vec::<(i32, i32)>::new();

    let i:i32 = 0;

    for i in 0..amount {
        edges.push((left, right));
    }

    edges
}

fn create_branches(amount:i32) -> Vec<XFlowBranch> {
    let left   = get_rand();
    let right  = get_rand();

    let mut branches = Vec::<XFlowBranch>::new();

    let i:i32 = 0;

    for i in 0..amount {
        branches.push(
            XFlowBranch {
                name: "Some branch".to_string(),
                edge : (left, right)
            })
    }

    branches
}

fn create_xflow_struct() -> XFlowStruct {
    XFlowStruct {
        id:       1,
        version:  1,
        name:     "Some name".to_string(),
        nodes:    create_nodes(5),
        edges:    create_edges(5),
        branches: create_branches(5)
    }
}

#[test]
fn test_xfs() {
    let xfs:XFlowStruct = XFlowStruct::new();
    println!("Hello, xflow {:?}", xfs.to_string());
    assert_eq!(xfs.nodes.len(), 5);
    assert_eq!(xfs.edges.len(), 5);
    assert_eq!(xfs.branches.len(), 5);
}
