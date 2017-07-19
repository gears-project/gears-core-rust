use std::collections::HashSet;

use super::common::Document;

pub type XFlowDocument = Document<XFlow>;
pub type XFlowEdge = (i32, i32);

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
// partof: SPC-serialization-json
pub struct XFlow {
    pub requirements: Vec<XFlowRequirement>,
    pub variables: XFlowVariables,
    pub nodes: Vec<XFlowNode>,
    pub edges: Vec<XFlowEdge>,
    pub branches: Vec<XFlowBranch>,
}

#[derive(Debug)]
pub enum XFlowError {
    NoEntryNode,
    NoTerminalNode,
    MultipleEntryNodes,
    DuplicateNodeIDs,
    NodeNotFound,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
// partof: #SPC-serialization-json
pub enum XFlowValueType {
    #[serde(rename = "string")]
    String,
    #[serde(rename = "number")]
    Integer,
    #[serde(rename = "boolean")]
    Boolean,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
#[serde(untagged)]
pub enum XFlowValue {
    String(String),
    Integer(i64),
    Boolean(bool),
}

impl XFlowValue {
    pub fn string_value(&self) -> String {
        match *self {
            XFlowValue::String(ref s) => s.clone(),
            XFlowValue::Integer(ref i) => i.to_string(),
            XFlowValue::Boolean(ref b) => {
                match *b {
                    true => "true".to_owned(),
                    false => "false".to_owned(),
                }
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowRequirement {
    pub xtype: XFlowNodeType,
    pub version: i32,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowVariableDefinition {
    pub name: String,
    pub vtype: XFlowValueType,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowVariable {
    pub name: String,
    pub vtype: XFlowValueType,
    pub value: XFlowValue,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowVariables {
    pub input: Vec<XFlowVariableDefinition>,
    pub local: Vec<XFlowVariable>,
    pub output: Vec<XFlowVariableDefinition>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct XFlowNode {
    pub id: i32,
    pub nodetype: XFlowNodeType,
    pub action: String,
    pub label: String,
    pub parameters: XFlowNodeParameters,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Hash, Eq)]
pub enum XFlowNodeType {
    #[serde(rename = "flow")]
    Flow,
    #[serde(rename = "flox")]
    Flox,
    #[serde(rename = "call")]
    Call,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum XFlowNodeParameters {
    #[serde(rename = "flow")]
    Flow(FlowParameters),
    #[serde(rename = "flox")]
    Flox(FloxParameters),
    #[serde(rename = "call")]
    Call(CallParameters),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FlowParameters {}

impl Default for FlowParameters {
    fn default() -> Self {
        FlowParameters {}
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct FloxParameters {
    pub expression: String,
    pub returns: XFlowVariableDefinition,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct CallParameters {}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct XFlowBranch {
    pub edge: XFlowEdge,
    pub xvar: XFlowVariable,
}

impl XFlow {
    /// Get `XFlowNode`s of `nodetype` and `action`
    ///
    /// # Example
    /// ```
    /// use gears::structure::xflow::{XFlow, XFlowNodeType};
    /// let xfs = XFlow::default();
    /// let nodes = xfs.get_nodes_by(&XFlowNodeType::Flow, "start");
    /// assert_eq!(nodes.len(), 1);
    /// ```
    pub fn get_nodes_by(&self, nodetype: &XFlowNodeType, action: &str) -> Vec<&XFlowNode> {

        self.nodes
            .iter()
            .filter({
                        |node| node.nodetype == *nodetype && node.action == action
                    })
            .collect()

    }

    /// Get `XFlowNode`s of `nodetype`
    ///
    /// # Example
    /// ```
    /// use gears::structure::xflow::{XFlow, XFlowNodeType};
    /// let xfs = XFlow::default();
    /// let nodes = xfs.get_nodes_of_type(&XFlowNodeType::Flow);
    /// assert_eq!(nodes.len(), 2);
    /// ```
    pub fn get_nodes_of_type(&self, nodetype: &XFlowNodeType) -> Vec<&XFlowNode> {

        self.nodes
            .iter()
            .filter({
                        |node| node.nodetype == *nodetype
                    })
            .collect()
    }

    /// Get a `HashSet` of all variable names in input, local and output
    ///
    /// # Example
    /// ```
    /// use gears::structure::xflow::{XFlow};
    /// let xfs = XFlow::default();
    /// let names = xfs.get_all_variable_names();
    /// assert_eq!(names.len(), 0);
    /// ```
    pub fn all_variable_names(&self) -> HashSet<String> {
        let mut vars = HashSet::<String>::new();

        for xvar in &self.variables.input {
            vars.insert(xvar.name.clone());
        }

        for xvar in &self.variables.local {
            vars.insert(xvar.name.clone());
        }

        for xvar in &self.variables.output {
            vars.insert(xvar.name.clone());
        }

        vars
    }

    pub fn get_in_edges(&self, node: &XFlowNode) -> Vec<&XFlowEdge> {

        self.edges
            .iter()
            .filter({
                        |edge| edge.1 == node.id
                    })
            .collect()
    }

    pub fn get_out_edges(&self, node: &XFlowNode) -> Vec<&XFlowEdge> {

        self.edges
            .iter()
            .filter({
                        |edge| edge.0 == node.id
                    })
            .collect()
    }

    pub fn get_branches_for(&self, edge: &XFlowEdge) -> Vec<&XFlowBranch> {

        self.branches
            .iter()
            .filter({
                        |branch| edge.0 == branch.edge.0 && edge.1 == branch.edge.1
                    })
            .collect()
    }

    pub fn get_out_branches(&self, id: i32) -> Vec<&XFlowBranch> {

        self.branches
            .iter()
            .filter({
                        |branch| branch.edge.0 == id
                    })
            .collect()
    }

    pub fn get_entry_node(&self) -> Result<&XFlowNode, XFlowError> {
        let res = self.get_nodes_by(&XFlowNodeType::Flow, "start");
        match res.len() {
            0 => Err(XFlowError::NoEntryNode),
            1 => Ok(res[0]),
            _ => Err(XFlowError::MultipleEntryNodes),
        }
    }

    pub fn get_terminal_nodes(&self) -> Result<Vec<&XFlowNode>, XFlowError> {
        let res = self.get_nodes_by(&XFlowNodeType::Flow, "end");
        match res.len() {
            0 => Err(XFlowError::NoTerminalNode),
            _ => Ok(res),
        }
    }

    pub fn get_node_id(&self, id: i32) -> Option<&XFlowNode> {
        let nodes: Vec<&XFlowNode> = self.nodes
            .iter()
            .filter({
                        |node| node.id == id
                    })
            .collect();

        match nodes.len() {
            1 => {
                match nodes.first() {
                    Some(node) => Some(node),
                    None => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_all_variable_names(&self) -> HashSet<String> {
        let mut names = HashSet::<String>::new();

        for xvar in &self.variables.local {
            if !names.contains(&xvar.name) {
                names.insert(xvar.name.clone());
            }
        }

        for xvar in &self.variables.input {
            if !names.contains(&xvar.name) {
                names.insert(xvar.name.clone());
            }
        }

        for xvar in &self.variables.output {
            if !names.contains(&xvar.name) {
                names.insert(xvar.name.clone());
            }
        }

        names
    }
}

impl Default for XFlow {
    /// Constructs a new `XFlow`
    ///
    /// # Example
    /// ```
    /// use gears::structure::xflow::{XFlow};
    /// let xfs = XFlow::default();
    /// println!("XFlow has {} requirements", xfs.requirements.len());
    /// ```
    fn default() -> Self {

        let mut nodes = Vec::<XFlowNode>::new();
        nodes.push(XFlowNode {
                       id: 1,
                       nodetype: XFlowNodeType::Flow,
                       action: "start".to_owned(),
                       label: "Start".to_owned(),
                       parameters: XFlowNodeParameters::Flow(FlowParameters::default()),
                   });
        nodes.push(XFlowNode {
                       id: 2,
                       nodetype: XFlowNodeType::Flow,
                       action: "end".to_owned(),
                       label: "End".to_owned(),
                       parameters: XFlowNodeParameters::Flow(FlowParameters::default()),
                   });

        let mut edges = Vec::<XFlowEdge>::new();
        edges.push((1, 2));

        let mut requirements = Vec::<XFlowRequirement>::new();
        requirements.push(XFlowRequirement {
                              xtype: XFlowNodeType::Flow,
                              version: 1,
                          });

        XFlow {
            requirements: requirements,
            variables: XFlowVariables {
                input: Vec::<XFlowVariableDefinition>::new(),
                local: Vec::<XFlowVariable>::new(),
                output: Vec::<XFlowVariableDefinition>::new(),
            },
            nodes: nodes,
            edges: edges,
            branches: Vec::<XFlowBranch>::new(),
        }
    }
}
