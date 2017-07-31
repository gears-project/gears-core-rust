use runtime::actiondispatch::dispatchable::*;
use structure::xflow::*;
use xfstate::XFState;
use flox;

#[derive(Serialize, Deserialize, Debug)]
struct FloxParameters {
    expression: String,
    returns: XFlowVariableDefinition,
}

pub struct Flox;

impl Flox {
    fn process_node(&self, node: &XFlowNode, state: &mut XFState) -> () {
        debug!("Flox: {} - {}", node.id, state);
        match node.parameters {
            XFlowNodeParameters::Flox(ref node_params) => {
                match node.action.as_ref() {
                    "evalexpr" => {
                        debug!(
                            "Flox: evalexpr: '{}' - state: '{}' - params: '{:?}'",
                            node.id,
                            state,
                            node_params
                        );
                        let expr = node_params.expression.as_str();
                        debug!("Expression: '{}'", expr);
                        match flox::parse_context(node_params.expression.as_str(), state) {
                            Ok(res) => {
                                debug!("Expression: '{}' - Result: '{:?}'", expr, res);
                                state.add(&XFlowVariable {
                                    name: node_params.returns.name.clone(),
                                    vtype: node_params.returns.vtype.clone(),
                                    value: res.clone(),
                                });
                            }
                            Err(err) => {
                                error!("Expression: '{}' - Result: '{:?}'", expr, err);
                            }

                        }
                    }
                    _ => {
                        error!(
                            "Unimplemented/unhandled action id: '{}' - state: '{}'",
                            node.id,
                            state
                        );
                    }
                }
            }
            _ => {
                error!(
                    "Incorrect NodeType dispatched to Flox processor {:?}!",
                    node
                );
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
