use super::common::{Document, I18NString};
use jsonapi::model::*;

pub type ModelConfigDocument = Document<ModelConfig>;
jsonapi_model!(ModelConfigDocument; "config");

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[derive(Getable, Pushable, VmType)]
#[gluon(vm_type = "gears.modelconfig")]
pub struct ModelConfig {
    pub default_locale: String,
    pub active_locale: String,
    pub locales: Vec<String>,
}

impl Default for ModelConfig {
    fn default() -> Self {
        ModelConfig {
            default_locale: "en_US".to_owned(),
            active_locale: "en_US".to_owned(),
            locales: Vec::<String>::new(),
        }
    }
}


