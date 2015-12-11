#[derive(Debug)]

enum DocType {
    XFlowStruct,
}

pub struct DocStore {
    docs: Vec<(String, DocType)>,
}

impl DocStore {
    pub fn new() -> DocStore {
        DocStore { docs: Vec::<(String, DocType)>::new() }
    }

    pub fn add(&self, doc: DocType) {}
}
