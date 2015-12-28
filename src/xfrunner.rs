use xfstruct::*;
use xfstate::*;
use dispatcher::*;

#[derive(Debug)]
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
    status: XFlowStatus,
    xflow: &'a XFlowStruct,
    dispatcher: &'a Dispatcher<'a>,
    state: XFState,
    current_node: Option<&'a XFlowNode>,
}

impl<'a> XFlowRunner<'a> {
    pub fn new(xflow: &'a XFlowStruct, dispatcher: &'a Dispatcher<'a>) -> XFlowRunner<'a> {

        let mut state = XFState::new();

        for xvar in &xflow.variables.input {
            state.add(&xvar);
        }

        for xvar in &xflow.variables.local {
            state.add(&xvar);
        }

        match xflow.get_entry_node() {
            Ok(node) => {
                XFlowRunner {
                    status: XFlowStatus::Initialized,
                    xflow: xflow,
                    dispatcher: dispatcher,
                    state: state,
                    current_node: Some(node),
                }
            }
            _ => {
                XFlowRunner {
                    status: XFlowStatus::Uninitialized,
                    xflow: xflow,
                    dispatcher: dispatcher,
                    state: state,
                    current_node: None,
                }
            }
        }
    }

    pub fn can_run(&self) -> bool {
        match self.status {
            XFlowStatus::Initialized | XFlowStatus::Running => true,
            _ => false,
        }
    }

    pub fn is_initialized(&self) -> bool {
        match self.status {
            XFlowStatus::Initialized => true,
            _ => false,
        }
    }

    pub fn is_completed_ok(&self) -> bool {
        match self.status {
            XFlowStatus::Finished => true,
            _ => false,
        }
    }

    pub fn run(&mut self) -> () {
        while self.can_run() {
            self.step();
        }
    }

    pub fn step(&mut self) -> bool {
        self.next_node();
        self.run_node()
    }

    fn run_node(&mut self) -> bool {
        let st = &mut self.state;
        if let Some(node) = self.current_node {
            self.status = XFlowStatus::Running;
            self.dispatcher.dispatch(node, st);
            true
        } else {
            self.status = XFlowStatus::Finished;
            false
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
                        0 => {
                            self.status = XFlowStatus::InvalidState;
                            self.current_node = None;
                        }
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
}
