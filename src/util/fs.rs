use structure::common::DocumentHeader;
use structure::model::{ModelDocument, ModelConfigDocument};
use structure::xflow::XFlowDocument;
use structure::page::PageDocument;
use structure::domain::DomainDocument;
use structure::translation::TranslationDocument;

use generation;

use glob::glob_with;
use glob::MatchOptions;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std;

fn read_json_file(path: &Path) -> String {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ModelLoadError {
    NoPath,
    Unhandled,
}

fn write_file(filename: &str, data: &str) -> () {
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

fn create_dir(path: &str) -> () {
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

pub fn build_dotfiles(model: &ModelDocument, path: &str) -> Result<(), ModelLoadError> {
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

    for xflow in &model.body.xflows {
        let doc = generation::xflow_to_dot::output(&xflow);

        let filename = format!("{path}/{id}.dot", path = xflow_path, id = xflow.id);
        write_file(&filename, &doc);
    }

    let doc = generation::domain_to_dot::output(&model.body.domain);

    let filename = format!("{path}/domain.dot", path = path);
    write_file(&filename, &doc);

    Ok(())
}

pub fn build_to_react_app(model: &ModelDocument, path: &str) -> Result<(), ModelLoadError> {
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

    for page in &model.body.pages {
        let doc = generation::page_to_react_component::output_html(&page);

        let filename = format!("{path}/{id}.js", path = component_path, id = page.id);
        write_file(&filename, &doc);

    }

    for xflow in &model.body.xflows {
        let doc = generation::xflow_to_es5::output(&xflow);

        let filename = format!("{path}/{id}.js", path = xflow_path, id = xflow.id);
        write_file(&filename, &doc);
    }

    Ok(())
}

pub fn model_to_fs(model: &ModelDocument, path: &str) -> Result<(), ModelLoadError> {
    // partof: #SPC-serialization-fs

    // XXX Error handling, assumption checking

    debug!(
        "Writing model id:'{}', version:'{}' to directory '{}'",
        model.id,
        model.version,
        path
    );

    let model_header_doc_filename = format!("{}/model.json", path);
    write_file(&model_header_doc_filename, &model.get_header().to_json());

    let model_config_doc_filename = format!("{}/config.json", path);
    write_file(&model_config_doc_filename, &model.body.config.to_json());

    let doc_filename = format!("{}/domain.json", path);
    write_file(&doc_filename, &model.body.domain.to_json());

    let xflows_path_name = format!("{}/xflows", path);
    create_dir(&xflows_path_name);

    for doc in &model.body.xflows {
        let doc_filename = format!("{}/{}.json", xflows_path_name, doc.id);
        write_file(&doc_filename, &doc.to_json());
    }

    let pages_path_name = format!("{}/pages", path);
    create_dir(&pages_path_name);

    for doc in &model.body.pages {
        let doc_filename = format!("{}/{}.json", pages_path_name, doc.id);
        write_file(&doc_filename, &doc.to_json());
    }

    let translations_path_name = format!("{}/translations", path);
    create_dir(&translations_path_name);

    for doc in &model.body.translations {
        let doc_filename = format!("{}/{}.json", translations_path_name, doc.body.locale);
        write_file(&doc_filename, &doc.to_json());
    }

    Ok(())
}

pub fn model_from_fs(path: &str) -> Result<ModelDocument, ModelLoadError> {
    // partof: SPC-serialization-fs

    // XXX Error handling, assumption checking

    debug!("Reading model from directory '{}'", path);
    let model_header_filename = format!("{}/model.json", path);
    let model_header_path = Path::new(&model_header_filename);
    let model_header_json = read_json_file(model_header_path);
    let model_header: DocumentHeader = DocumentHeader::from_json(&model_header_json);

    let mut modeldoc = ModelDocument::new_from_header(&model_header);

    let model_config_filename = format!("{}/config.json", path);
    let model_config_path = Path::new(&model_config_filename);
    let model_config_json = read_json_file(model_config_path);
    let model_config: ModelConfigDocument = ModelConfigDocument::from_json(&model_config_json);

    modeldoc.body.config = model_config;

    let domain_filename = format!("{}/domain.json", path);
    let domain_path = Path::new(&domain_filename);
    let json = read_json_file(domain_path);
    let domain: DomainDocument = DomainDocument::from_json(&json);

    modeldoc.body.domain = domain;


    let glob_options = MatchOptions {
        case_sensitive: true,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    let xflow_files_path = format!("{}/xflows/*", path);
    for item in glob_with(&xflow_files_path, &glob_options).unwrap() {
        if let Ok(path) = item {
            let json = read_json_file(&path);
            let xflow_doc: XFlowDocument = XFlowDocument::from_json(&json);
            modeldoc.body.xflows.push(xflow_doc);
        } else {
            warn!("Unable to load doc from '{:?}'", item);
        }
    }

    let page_files_path = format!("{}/pages/*", path);
    for item in glob_with(&page_files_path, &glob_options).unwrap() {
        if let Ok(path) = item {
            let json = read_json_file(&path);
            let page_doc: PageDocument = PageDocument::from_json(&json);
            modeldoc.body.pages.push(page_doc);
        } else {
            warn!("Unable to load doc from '{:?}'", item);
        }
    }

    let translation_files_path = format!("{}/translations/*", path);
    for item in glob_with(&translation_files_path, &glob_options).unwrap() {
        if let Ok(path) = item {
            let json = read_json_file(&path);
            let translation_doc: TranslationDocument = TranslationDocument::from_json(&json);
            modeldoc.body.translations.push(translation_doc);
        } else {
            warn!("Unable to load doc from '{:?}'", item);
        }
    }

    Ok(modeldoc)
}

pub fn init_new_model_dir(path: &str) -> Result<(), ModelLoadError> {
    create_dir(path);
    let mut model = ModelDocument::default();
    let default_locale = model.body.config.body.default_locale.clone();
    model.add_locale(&default_locale);
    model.pad_all_translations();
    model_to_fs(&model, &path)
}

pub fn is_model_dir(path: &str) -> bool {
    match model_from_fs(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
