use crate::structure::common::{DocumentHeader, ModelLoadError, DocumentNature, DocumentFileSystemLoadable};
use crate::structure::gxmodel::GxModel;
use crate::structure::model::ModelDocument;

use crate::generation;

use glob::glob_with;
use glob::MatchOptions;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std;

pub(crate) fn read_json_file(path: &Path) -> String {
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, Error::description(&why));
    };

    s
}

pub(crate) fn write_file(filename: &str, data: &str) -> () {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => {
            error!("couldn't create {}: {}", display, why.description());
            panic!("couldn't create {}: {}", display, why.description());
        }
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => {
            error!("couldn't write to {}: {}", display, why.description());
            panic!("couldn't write to {}: {}", display, why.description());
        }
        Ok(_) => debug!("successfully wrote to {}", display),
    }
}

pub(crate) fn create_dir(path: &str) -> () {
    debug!("Creating directory '{:?}'", path);
    if !Path::new(path).exists() {
        match std::fs::create_dir(&path) {
            Ok(_) => {
                debug!("Created directory '{:?}' : OK", path);
            }
            Err(_) => {
                error!("Error creating directory '{:?}'", path);
            }
        };
    } else {
        debug!("Directory '{:?}' exists, not creating", path);
    }
}

pub fn build_dotfiles(model: &GxModel, path: &str) -> Result<(), ModelLoadError> {
    // partof: #SPC-artifact-generation-model

    // XXX Error handling, assumption checking

    debug!(
        "Building dotfiles id:'{}' assets, model version:'{}' in directory '{}'",
        model.id,
        model.version,
        path
    );

    create_dir(&path);
    let xflow_path = format!("{path}/xflows", path = path);
    create_dir(&xflow_path);

    for xflow in &model.xflows {
        let doc = generation::xflow_to_dot::output(&xflow);

        let filename = format!("{path}/{id}.dot", path = xflow_path, id = xflow.id);
        write_file(&filename, &doc);
    }

    let doc = generation::domain_to_dot::output(&model.domain);

    let filename = format!("{path}/domain.dot", path = path);
    write_file(&filename, &doc);

    Ok(())
}

pub fn build_to_react_app(model: &GxModel, path: &str) -> Result<(), ModelLoadError> {
    // partof: SPC-artifact-generation-model

    // XXX Error handling, assumption checking

    debug!(
        "Building id:'{}' assets, model version:'{}' in directory '{}'",
        model.id,
        model.version,
        path
    );

    create_dir(&path);
    let xflow_path = format!("{path}/xflows", path = path);
    create_dir(&xflow_path);
    let component_path = format!("{path}/components", path = path);
    create_dir(&component_path);

    for page in &model.pages {
        let doc = generation::page_to_react_component::output_html(&page);

        let filename = format!("{path}/{id}.js", path = component_path, id = page.id);
        write_file(&filename, &doc);

    }

    for xflow in &model.xflows {
        let doc = generation::xflow_to_es5::output(&xflow);

        let filename = format!("{path}/{id}.js", path = xflow_path, id = xflow.id);
        write_file(&filename, &doc);
    }

    Ok(())
}

pub fn load_docheader_from_filename(filename: &str) -> Result<DocumentHeader, ModelLoadError>
    {
    let path = Path::new(&filename);
    let json = read_json_file(path);
    debug!("load_doc_from_filename : Deserializing JSON from {}", filename);
    DocumentHeader::from_json(&json)
}

pub fn load_doc_from_filename<T>(filename: &str) -> Result<T, ModelLoadError>
where T : DocumentNature<Doc = T>
    {
    let path = Path::new(&filename);
    let json = read_json_file(path);
    debug!("load_doc_from_filename : Deserializing JSON from {}", filename);
    <T>::from_json(&json)
}

pub fn load_docs_from_path<T>(path: &str, glob_options: &MatchOptions) -> Result<Vec<T>, ModelLoadError>
where T : DocumentNature<Doc = T>
    {

    let mut docs = Vec::<T>::new();
    if let Ok(coll) = glob_with(&path, &glob_options) {
        for item in coll {
            if let Ok(path) = item {
                let json = read_json_file(&path);
                debug!("load_docs_from_path : Deserializing JSON from {:?}", path);
                let mut doc = match <T>::from_json(&json) {
                    Ok(res) => res,
                    Err(err) => return Err(err)
                };
                docs.push(doc);
            } else {
                warn!("Unable to load doc from '{:?}'", item);
            }
        }
    } else {
        error!("load_docs_from_patth : unable to read from path : {}", path);
        panic!(format!("model_from_fs : unable to read from path : {}", path));
    }

    Ok(docs)
}

pub fn init_new_model_dir(path: &str) -> Result<(), ModelLoadError> {
    create_dir(path);
    let mut model = ModelDocument::default();
    let default_locale = model.body.config.body.default_locale.clone();
    model.add_locale(&default_locale);
    model.pad_all_translations();
    model.write_to_filesystem(&path)
}

pub fn init_new_gxmodel_dir(path: &str) -> Result<(), ModelLoadError> {
    create_dir(path);
    let model = GxModel::default();
    model.write_to_filesystem(&path)
}

pub fn is_model_dir(path: &str) -> bool {
    match ModelDocument::load_from_filesystem(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn is_gxmodel_dir(path: &str) -> bool {
    match GxModel::load_from_filesystem(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
