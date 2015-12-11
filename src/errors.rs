#[derive(Debug)]
pub enum XFlowError {
    NoEntryNode,
    NoTerminalNode,
    MultipleEntryNodes,
}
