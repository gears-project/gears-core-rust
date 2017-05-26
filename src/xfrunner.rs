use xfstruct::*;
use xfstate::*;
use dispatcher::*;

#[derive(Debug, PartialEq)]
pub enum XFlowStatus {
    Uninitialized,
    Initialized,
    Running,
    Finished,
    Aborted,
    TimedOut,
    InvalidState,
}

pub struct XFlowRunner<'a> {
    pub status: XFlowStatus,
    xflow: &'a XFlowStruct,
    dispatcher: &'a Dispatcher<'a>,
    state: XFState,
    current_node: Option<&'a XFlowNode>,
    pub output: Option<Vec<XFlowValue>>,
}

impl<'a> XFlowRunner<'a> {
    pub fn new(xflow: &'a XFlowStruct, dispatcher: &'a Dispatcher<'a>) -> XFlowRunner<'a> {

        let mut state = XFState::default();

        // for xvar in &xflow.variables.input {
        //     state.add(xvar);
        // }

        for xvar in &xflow.variables.local {
            state.add(xvar);
        }

        match xflow.get_entry_node() {
            Ok(node) => {
                XFlowRunner {
                    status: XFlowStatus::Initialized,
                    xflow: xflow,
                    dispatcher: dispatcher,
                    state: state,
                    current_node: Some(node),
                    output: None,
                }
            }
            _ => {
                XFlowRunner {
                    status: XFlowStatus::Uninitialized,
                    xflow: xflow,
                    dispatcher: dispatcher,
                    state: state,
                    current_node: None,
                    output: None,
                }
            }
        }
    }

    pub fn new_with_input(xflow: &'a XFlowStruct,
                          dispatcher: &'a Dispatcher<'a>,
                          input: &'a XFState)
                          -> Result<XFlowRunner<'a>, String> {

        let mut state = XFState::default();

        for (_, xvar) in &input.store {
            state.add(xvar);
        }

        for xvar in &xflow.variables.local {
            state.add(xvar);
        }

        match xflow.get_entry_node() {
            Ok(node) => {
                Ok(XFlowRunner {
                    status: XFlowStatus::Initialized,
                    xflow: xflow,
                    dispatcher: dispatcher,
                    state: state,
                    current_node: Some(node),
                    output: None,
                })
            }
            _ => Err("Unable to init XFlowRunner".to_owned()),
        }
    }

    pub fn can_run(&self) -> bool {
        match self.status {
            XFlowStatus::Initialized | XFlowStatus::Running => true,
            _ => false,
        }
    }

    pub fn is_initialized(&self) -> bool {
        self.status == XFlowStatus::Initialized
    }

    pub fn is_completed(&self) -> bool {
        self.status == XFlowStatus::Finished || self.status == XFlowStatus::Aborted ||
        self.status == XFlowStatus::TimedOut || self.status == XFlowStatus::InvalidState
    }

    pub fn is_completed_ok(&self) -> bool {
        self.status == XFlowStatus::Finished
    }

    pub fn run(&mut self) -> () {
        while self.can_run() {
            self.step();
        }
    }

    pub fn step(&mut self) -> () {
        self.next_node();
        self.run_node();
    }

    fn run_node(&mut self) -> () {
        let st = &mut self.state;
        if let Some(node) = self.current_node {
            self.status = XFlowStatus::Running;
            self.dispatcher.dispatch(node, st);
        } else {
            self.status = XFlowStatus::Finished;
        }
    }

    fn next_node(&mut self) -> () {
        if let Some(current_node) = self.current_node {
            let edges = self.xflow.get_out_edges(current_node);
            match edges.len() {
                0 => {
                    self.status = XFlowStatus::InvalidState;
                    self.current_node = None;
                }
                1 => {
                    if let Some(edge) = edges.first() {
                        self.current_node = self.xflow.get_node_id(edge.1);
                    } else {
                        self.status = XFlowStatus::InvalidState;
                        self.current_node = None;
                    }
                }
                _ => {

                    // XXX This branch matching is sloppy - it should happen on the node.parameters
                    // settings
                    //

                    let branches: Vec<&XFlowBranch> = self.xflow
                        .get_out_branches(current_node.id)
                        .iter()
                        .filter({
                            |branch| {
                                let xv = self.state
                                    .get(&branch.xvar
                                        .name);
                                if let Some(xvar) = xv {
                                    *xvar == branch.xvar
                                } else {
                                    false
                                }
                            }
                        })
                        .cloned()
                        .collect();
                    match branches.len() {
                        1 => {
                            if let Some(branch) = branches.first() {
                                self.current_node = self.xflow.get_node_id(branch.edge.1);
                            } else {
                                self.status = XFlowStatus::InvalidState;
                                self.current_node = None;
                            }
                        }
                        _ => {
                            self.status = XFlowStatus::InvalidState;
                            self.current_node = None;
                        }
                    }
                }
            }
        } else {
            self.status = XFlowStatus::InvalidState;
            self.current_node = None;
        }
    }

    pub fn get_output(self) -> Result<XFState, String> {
        if self.status == XFlowStatus::Finished {
            let mut state = XFState::default();
            for xvar_out in &self.xflow.variables.output {
                if let Some(xvar_local) = self.state.get(&xvar_out.name) {
                    if xvar_local.vtype == xvar_out.vtype {
                        state.add(xvar_local);
                    } else {
                        error!("Output var '{}' has a different type than its local one",
                               &xvar_out.name);
                        return Err(format!("Output var '{}' has a different type than its local \
                                            one",
                                           &xvar_out.name));
                    }
                } else {
                    error!("Required var '{:?}' not found in state!", &xvar_out.name);
                    return Err(format!("Required var '{}' not found in state!", &xvar_out.name));
                }
            }
            Ok(state)
        } else {
            error!("Called before xflow has finished!");
            Err("Called before xflow has finished!".to_owned())
        }
    }
}
