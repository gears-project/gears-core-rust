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
                info!("Flox: evalexpr: '{}' - state: '{}' - params: '{:?}'",
                      node.id,
                      state,
                      node_params);
                let expr = node_params.expression.as_str();
                debug!("Expression: '{}'", expr);
                match flox::parse_context(node_params.expression.as_str(), &state) {
                    Ok(res) => {
                        debug!("Expression: '{}' - Result: '{:?}'", expr, res);
                        state.add(&XFlowVariable {
                            name: node_params.returns.name,
                            vtype: node_params.returns.vtype,
                            value: res.clone(),
                        });
                    }
                    Err(err) => {
                        error!("Expression: '{}' - Result: '{:?}'", expr, err);
                    }

                }
            }
            _ => {
                error!("Unimplemented/unhandled action id: '{}' - state: '{}'",
                       node.id,
                       state);
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
