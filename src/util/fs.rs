use structure::model::ModelDocument;

pub enum Error {
    NoPath,
    Unhandled,
}

pub fn model_to_fs(path: &str) -> Result<(), Error> {
    Ok(())
}

pub fn model_from_fs() -> ModelDocument {
    let mut modeldoc = ModelDocument::default();
    modeldoc.version = 2;

    modeldoc
}
