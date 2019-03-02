use std::collections::HashMap;
use uuid::Uuid;
use jsonapi::model::*;
use jsonapi::array::JsonApiArray;

use glob::glob_with;
use glob::MatchOptions;

use super::common::{DocumentHeader, DocumentNature, ModelLoadError, DocumentFileSystemLoadable};

use super::modelconfig::ModelConfigDocument;
use super::domain::DomainDocument;
use super::xflow::{XFlowDocument, XFlowDocumentList};
use super::page::{PageDocument, PageDocumentList};
use super::translation::{TranslationDocument, TranslationDocumentList};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
// #[cfg_attr(feature = "gluon", derive(Getable, Pushable, VmType))]
// #[cfg_attr(feature = "gluon", gluon(vm_type = "gears.gxmodel"))]
pub struct GxModel {
    pub id: Uuid,
    pub name: String,
    pub doctype: String,
    pub doctype_version: i64,
    pub version: i64,
    pub config: ModelConfigDocument,
    pub domain: DomainDocument,
    pub xflows: XFlowDocumentList,
    pub pages: PageDocumentList,
    pub translations: TranslationDocumentList,
}
jsonapi_model!(GxModel; "gxmodel"; has one domain, config; has many xflows, pages, translations);

impl Default for GxModel {
    fn default() -> Self {
        GxModel {
            id: Uuid::new_v4(),
            name: "default".to_owned(),
            doctype: "".to_owned(),
            doctype_version: 1,
            version: 1,
            config: ModelConfigDocument::default(),
            domain: DomainDocument::default(),
            xflows: XFlowDocumentList::new(),
            pages: PageDocumentList::new(),
            translations: TranslationDocumentList::new(),
        }
    }
}

impl DocumentNature for GxModel {
    type Doc = GxModel;

    fn new_from_header(header: &DocumentHeader) -> Self {
        Self {
            id: header.id.clone(),
            name: header.name.clone(),
            doctype: header.doctype.clone(),
            doctype_version: header.doctype_version.clone(),
            version: header.version.clone(),
            config: ModelConfigDocument::default(),
            domain: DomainDocument::default(),
            xflows: XFlowDocumentList::new(),
            pages: PageDocumentList::new(),
            translations: TranslationDocumentList::new(),
        }
    }

    fn get_header(&self) -> DocumentHeader {
        DocumentHeader {
            id: self.id.clone(),
            name: self.name.clone(),
            doctype: self.doctype.clone(),
            doctype_version: self.doctype_version.clone(),
            version: self.version.clone(),
        }
    }

    fn set_header(&mut self, header: &DocumentHeader) -> () {
        self.id = header.id.clone();
        self.name = header.name.clone();
        self.doctype = header.doctype.clone();
        self.doctype_version = header.doctype_version.clone();
        self.version = header.version.clone();
    }

    /// Return a string representation of the Document
    ///
    fn to_string(&self) -> String {
        self.summary()
    }

    /// Return a summary of the Document
    ///
    fn summary(&self) -> String {
        format!("Doc {:?} - {:?} - {:?}", self.doctype, self.id, self.name)
    }

    /// Return an indented JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }

    /// Return a compact JSON representation of the Document
    ///
    /// partof: SPC-serialization-json
    fn to_json_compact(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    fn from_json(s: &str) -> Result<Self, ModelLoadError> {
        match serde_json::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }

    /// Update a Document from a JSON string
    ///
    /// partof: SPC-serialization-json
    fn update_from_json(&mut self, s: &str) -> Result<&Self, String> {
        let value = serde_json::from_str(s).unwrap();
        *self = serde_json::from_value(value).unwrap();
        Ok(self)
    }

    /// Return a YAML representation of the Document
    ///
    /// partof: #SPC-serialization-yaml
    fn to_yaml(&self) -> String {
        serde_yaml::to_string(&self).unwrap()
    }

    /// Initialize a Document from a JSON string
    ///
    /// partof: SPC-serialization-yaml
    fn from_yaml(s: &str) -> Result<Self, ModelLoadError> {
        match serde_yaml::from_str(s) {
            Ok(res) => Ok(res),
            Err(err) => {
                let msg = format!("{}", err);
                Err(ModelLoadError::BadStructure(msg))
            }
        }
    }

    /// Update a Document from a YAML string
    ///
    /// partof: SPC-serialization-yaml
    fn update_from_yaml(&mut self, s: &str) -> Result<&Self, String> {
        let value = serde_yaml::from_str(s).unwrap();
        *self = serde_yaml::from_value(value).unwrap();
        Ok(self)
    }

}

impl DocumentFileSystemLoadable for GxModel {
    type Doc = GxModel;

    fn load_from_filesystem(path: &str) -> Result<GxModel, ModelLoadError> {
        // partof: SPC-serialization-fs

        // XXX Error handling, assumption checking

        debug!("Reading model from directory '{}'", path);

        let model_header = ::util::fs::load_docheader_from_filename(&format!("{}/model.json", path))?;

        let mut gxmodel = GxModel::new_from_header(&model_header);

        let model_config = ::util::fs::load_doc_from_filename::<ModelConfigDocument>(
            &format!("{}/config.json", path)
            )?;

        gxmodel.config = model_config;

        let domain = ::util::fs::load_doc_from_filename::<DomainDocument>(
            &format!("{}/domain.json", path)
            )?;

        gxmodel.domain = domain;

        let glob_options = MatchOptions {
            case_sensitive: true,
            require_literal_separator: false,
            require_literal_leading_dot: false,
        };

        let mut docs = ::util::fs::load_docs_from_path::<XFlowDocument>(
            &format!("{}/xflows/*", path),
            &glob_options)?;

        gxmodel.xflows.append(&mut docs);

        let mut docs = ::util::fs::load_docs_from_path::<PageDocument>(
            &format!("{}/pages/*", path),
            &glob_options)?;
        gxmodel.pages.append(&mut docs);

        let mut docs = ::util::fs::load_docs_from_path::<TranslationDocument>(
            &format!("{}/translations/*", path),
            &glob_options)?;
        gxmodel.translations.append(&mut docs);

        Ok(gxmodel)
    }

    fn write_to_filesystem(&self, path: &str) -> Result<(), ModelLoadError> {
        // partof: #SPC-serialization-fs

        // XXX Error handling, assumption checking

        debug!(
            "Writing gxself id:'{}', version:'{}' to directory '{}'",
            self.id,
            self.version,
            path
        );

        let self_header_doc_filename = format!("{}/model.json", path);
        ::util::fs::write_file(&self_header_doc_filename, &self.get_header().to_json());

        let self_config_doc_filename = format!("{}/config.json", path);
        ::util::fs::write_file(&self_config_doc_filename, &self.config.to_json());

        let doc_filename = format!("{}/domain.json", path);
        ::util::fs::write_file(&doc_filename, &self.domain.to_json());

        let xflows_path_name = format!("{}/xflows", path);
        ::util::fs::create_dir(&xflows_path_name);

        for doc in &self.xflows {
            let doc_filename = format!("{}/{}.json", xflows_path_name, doc.id);
            ::util::fs::write_file(&doc_filename, &doc.to_json());
        }

        let pages_path_name = format!("{}/pages", path);
        ::util::fs::create_dir(&pages_path_name);

        for doc in &self.pages {
            let doc_filename = format!("{}/{}.json", pages_path_name, doc.id);
            ::util::fs::write_file(&doc_filename, &doc.to_json());
        }

        let translations_path_name = format!("{}/translations", path);
        ::util::fs::create_dir(&translations_path_name);

        for doc in &self.translations {
            let doc_filename = format!("{}/{}.json", translations_path_name, doc.body.locale);
            ::util::fs::write_file(&doc_filename, &doc.to_json());
        }

        Ok(())

    }

}

