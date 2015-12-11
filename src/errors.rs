#[derive(Debug)]
pub enum XFlowError {
    NoEntryNode,
    NoTerminalNode,
    MultipleEntryNodes,
}

#[derive(Debug)]
pub enum XFlowStatus {
    Completed,
    Aborted,
    TimedOut,
    InvalidState,
}
