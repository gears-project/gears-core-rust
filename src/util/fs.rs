use structure::model::ModelDocument;
use structure::xflow::XFlowDocument;
use structure::form::FormDocument;
use structure::domain::DomainDocument;

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
        Err(why) => panic!("couldn't create {}: {}", display, why.description()),
        Ok(file) => file,
    };

    match file.write_all(data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

pub fn model_to_fs(model: &ModelDocument, path: &str) -> Result<(), ModelLoadError> {

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

    let forms_path_name = format!("{}/forms", path);
    std::fs::create_dir(&forms_path_name).unwrap();

    for doc in &model.doc.forms {
        let doc_filename = format!("{}/{}.json", forms_path_name, doc.id);
        write_file(&doc_filename, &doc.to_json());
    }

    Ok(())
}

pub fn model_from_fs(path: &str) -> Result<ModelDocument, ModelLoadError> {
    let mut modeldoc = ModelDocument::default();
    modeldoc.version = 2;

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
            // XXX
        }
    }

    let form_files_path = format!("{}/forms/*", path);
    for item in glob_with(&form_files_path, &glob_options).unwrap() {
        if let Ok(path) = item {
            let json = read_json_file(&path);
            let form_doc: FormDocument = FormDocument::from_json(&json);
            modeldoc.doc.forms.push(form_doc);
        } else {
            // XXX
        }
    }

    let domain_filename = format!("{}/domain/domain.json", path);
    let domain_path = Path::new(&domain_filename);
    let json = read_json_file(&domain_path);
    let domain: DomainDocument = DomainDocument::from_json(&json);

    modeldoc.doc.domain = domain;

    Ok(modeldoc)
}
