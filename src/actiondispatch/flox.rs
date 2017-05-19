use serde_json;
use serde_json::Value;
use actiondispatch::dispatchable::*;
use xfstruct::*;
use xfstate::XFState;
use flox;

#[derive(Serialize, Deserialize, Debug)]
struct FloxParameters {
    expression: String,
    returns: XFlowVariableDefinition,
}

impl FloxParameters {
    pub fn from_optional_value(v: &Option<Value>) -> Result<FloxParameters, String> {
        let flox_params: FloxParameters = serde_json::from_value(v.clone().unwrap()).unwrap();
        Ok(flox_params)
    }
}

pub struct Flox;

impl Flox {
    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        debug!("Flox: {} - {}", node.id, state);
        let node_params = FloxParameters::from_optional_value(&node.parameters).unwrap();
        match node.action.as_ref() {
            "evalexpr" => {
                info!("Flox: evalexpr {} - {} - {:?}", node.id, state, node_params);
                match node.parameters {
                    Some(ref params) => {
                        match params.get("expression") {
                            Some(val) => {
                                debug!("Expression : {}", val);
                                match flox::parse(val.as_str().unwrap()) {
                                    Ok(res) => {
                                        debug!("Expression {} - Result - {:?}", val, res);
                                        state.add(&XFlowVariable {
                                            name: node_params.returns.name,
                                            vtype: node_params.returns.vtype,
                                            value: res.clone(),
                                        });
                                    }
                                    Err(err) => {
                                        error!("Expression {} - Result - {:?}", val, err);
                                    }
                                };
                            }
                            None => {
                                error!("No expression found in parameters");
                            }
                        }
                    }
                    None => {
                        error!("No parameters found in node");
                    }
                }
            }
            _ => {
                error!("Unimplemented/unhandled action : {} - {}", node.id, state);
            }

        }
    }
}

impl Default for Flox {
    fn default() -> Self {
        Flox {}
    }
}

impl Dispatchable for Flox {
    fn init(&mut self) -> Result<(), ()> {
        Ok(())
    }

    fn dispatch(&self, node: &XFlowNode, state: &mut XFState) -> Result<(), ()> {
        self.process_node(node, state);
        Ok(())
    }
}
