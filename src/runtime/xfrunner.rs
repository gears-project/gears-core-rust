use structure::xflow::*;
use xfstate::*;
use runtime::dispatcher::*;

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
    xflow: &'a XFlowDocument,
    dispatcher: &'a Dispatcher<'a>,
    state: XFState,
    current_node: Option<&'a XFlowNode>,
    pub output: Option<Vec<XFlowValue>>,
}

impl<'a> XFlowRunner<'a> {
    pub fn new(
        xflow: &'a XFlowDocument,
        dispatcher: &'a Dispatcher<'a>,
        input: &'a XFState,
    ) -> Result<XFlowRunner<'a>, String> {

        let mut state = XFState::default();

        for xvardef in &xflow.doc.variables.input {
            match input.get(&xvardef.name) {
                Some(xvar) => state.add(xvar),
                None => {
                    let err = format!(
                        "Missing required xvar in input parameters : {}",
                        xvardef.name
                    );
                    error!("{}", err);
                    return Err(err);
                }
            }
        }

        for xvar in &xflow.doc.variables.local {
            state.add(xvar);
        }

        match xflow.doc.get_entry_node() {
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

    pub fn run(&mut self) -> Result<(), String> {
        // XXX: Clean up error handling
        while self.can_run() {
            match self.step() {
                Ok(()) => (),
                Err(err) => {
                    error!("{}", err);
                }
            }
        }
        if self.is_completed_ok() {
            Ok(())
        } else {
            let msg = "Unhandled error has occurred while running flow".to_owned();
            Err(msg)
        }
    }

    pub fn step(&mut self) -> Result<(), String> {
        self.next_node();
        self.run_node()
    }

    fn run_node(&mut self) -> Result<(), String> {
        let st = &mut self.state;
        if let Some(node) = self.current_node {
            self.status = XFlowStatus::Running;
            match self.dispatcher.dispatch(node, st) {
                Ok(_) => Ok(()),
                Err(err) => Err(err),
            }
        } else {
            self.status = XFlowStatus::Finished;
            Ok(())
        }
    }

    fn next_node(&mut self) -> () {
        if let Some(current_node) = self.current_node {
            let edges = self.xflow.doc.get_out_edges(current_node);
            match edges.len() {
                0 => {
                    self.status = XFlowStatus::InvalidState;
                    self.current_node = None;
                }
                1 => {
                    if let Some(edge) = edges.first() {
                        self.current_node = self.xflow.doc.get_node_id(edge.1);
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
                        .doc
                        .get_out_branches(current_node.id)
                        .iter()
                        .filter({
                            |branch| {
                                let xv = self.state.get(&branch.xvar.name);
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
                                self.current_node = self.xflow.doc.get_node_id(branch.edge.1);
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
            for xvar_out in &self.xflow.doc.variables.output {
                if let Some(xvar_local) = self.state.get(&xvar_out.name) {
                    if xvar_local.vtype == xvar_out.vtype {
                        state.add(xvar_local);
                    } else {
                        let msg = format!(
                            "Output var '{}' has a different type than its local \
                                           one",
                            &xvar_out.name
                        );

                        error!("{}", msg);
                        return Err(msg);
                    }
                } else {
                    let msg = format!("Required var '{:?}' not found in state!", &xvar_out.name);
                    error!("{}", msg);
                    return Err(msg);
                }
            }
            Ok(state)
        } else {
            let msg = "Called before xflow has finished!".to_owned();
            error!("{}", msg);
            Err(msg)
        }
    }
}
