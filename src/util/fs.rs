use structure::model::{ModelDocument, ModelConfigDocument};
use structure::xflow::XFlowDocument;
use structure::page::PageDocument;
use structure::domain::DomainDocument;
use structure::translation::TranslationDocument;

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

pub fn model_to_fs(model: &ModelDocument, path: &str) -> Result<(), ModelLoadError> {
    // partof: #SPC-serialization-fs

    // XXX Error handling, assumption checking

    debug!("Writing model id:'{}', version:'{}' to directory '{}'",
           model.id,
           model.version,
           path);

    let model_config_doc_filename = format!("{}/config.json", path);
    let model_config_doc = &model.doc.config;
    write_file(&model_config_doc_filename, &model_config_doc.to_json());

    let domain_path_name = format!("{}/domain", path);
    std::fs::create_dir(&domain_path_name).unwrap();
    let doc_filename = format!("{}/domain.json", domain_path_name);
    let doc = &model.doc.domain;
    write_file(&doc_filename, &doc.to_json());

    let xflows_path_name = format!("{}/xflows", path);
    std::fs::create_dir(&xflows_path_name).unwrap();

    for doc in &model.doc.xflows {
        let doc_filename = format!("{}/{}.json", xflows_path_name, doc.id);
        write_file(&doc_filename, &doc.to_json());
    }

    let pages_path_name = format!("{}/pages", path);
    std::fs::create_dir(&pages_path_name).unwrap();

    for doc in &model.doc.pages {
        let doc_filename = format!("{}/{}.json", pages_path_name, doc.id);
        write_file(&doc_filename, &doc.to_json());
    }

    let translations_path_name = format!("{}/translations", path);
    std::fs::create_dir(&translations_path_name).unwrap();

    for doc in &model.doc.translations {
        let doc_filename = format!("{}/{}.json", translations_path_name, doc.id);
        write_file(&doc_filename, &doc.to_json());
    }

    Ok(())
}

pub fn model_from_fs(path: &str) -> Result<ModelDocument, ModelLoadError> {
    // partof: SPC-serialization-fs

    // XXX Error handling, assumption checking

    debug!("Reading model from directory '{}'", path);
    let mut modeldoc = ModelDocument::default();

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
            modeldoc.doc.xflows.push(xflow_doc);
        } else {
            warn!("Unable to load doc from '{:?}'", item);
        }
    }

    let page_files_path = format!("{}/pages/*", path);
    for item in glob_with(&page_files_path, &glob_options).unwrap() {
        if let Ok(path) = item {
            let json = read_json_file(&path);
            let page_doc: PageDocument = PageDocument::from_json(&json);
            modeldoc.doc.pages.push(page_doc);
        } else {
            warn!("Unable to load doc from '{:?}'", item);
        }
    }

    let translation_files_path = format!("{}/translations/*", path);
    for item in glob_with(&translation_files_path, &glob_options).unwrap() {
        if let Ok(path) = item {
            let json = read_json_file(&path);
            let translation_doc: TranslationDocument = TranslationDocument::from_json(&json);
            modeldoc.doc.translations.push(translation_doc);
        } else {
            warn!("Unable to load doc from '{:?}'", item);
        }
    }

    let model_config_filename = format!("{}/config.json", path);
    let model_config_path = Path::new(&model_config_filename);
    let model_config_json = read_json_file(&model_config_path);
    let model_config: ModelConfigDocument = ModelConfigDocument::from_json(&model_config_json);

    modeldoc.doc.config = model_config;

    let domain_filename = format!("{}/domain/domain.json", path);
    let domain_path = Path::new(&domain_filename);
    let json = read_json_file(&domain_path);
    let domain: DomainDocument = DomainDocument::from_json(&json);

    modeldoc.doc.domain = domain;

    Ok(modeldoc)
}

pub fn init_new_model_dir(path: &str) -> Result<(), ModelLoadError> {
    std::fs::create_dir(&path).unwrap();
    let model = ModelDocument::default();
    model_to_fs(&model, &path)
}

pub fn is_model_dir(path: &str) -> bool {
    match model_from_fs(&path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
