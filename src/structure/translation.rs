use super::common::{Document, DocumentList, I18NString};
use dsl::command::{GearsDsl, DslTree, DslToken, DslTokens, command_grammar};

use std::collections::{HashMap, BTreeMap};
use serde::{Serialize, Serializer};

pub type TranslationDocument = Document<Translation>;
pub type TranslationDocumentList = DocumentList<Translation>;

pub type TranslationMap = HashMap<String, I18NString>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Translation {
    pub locale: String,
    pub language: String,
    pub country: String,
    #[serde(serialize_with = "ordered_map")]
    pub items: TranslationMap,
}

fn ordered_map<S>(value: &HashMap<String, I18NString>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // partof: SPC-serialization-fs
    // Consistent serialization
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

impl Default for Translation {
    fn default() -> Self {
        Translation {
            locale: "".to_owned(),
            language: "".to_owned(),
            country: "".to_owned(),
            items: HashMap::<String, I18NString>::new(),
        }
    }
}

impl Translation {
    pub fn add(&mut self, key: &str, value: &str) -> () {
        self.items.insert(
            key.to_string().clone(),
            I18NString {
                locale: self.locale.clone(),
                key: key.to_string().clone(),
                value: value.to_string().clone(),
            },
        );
    }
    pub fn add_untranslated_from(&mut self, item: &I18NString) -> () {
        let value = format!("-untranslated-:{}", item.value);
        self.add(&item.key, &value);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TranslationCommand {
    Set(String, String),
    Add(String, String),
    Remove(String),
}

impl TranslationCommand {
    fn as_dsl_token(&self) -> DslToken {
        let s = match *self {
            TranslationCommand::Set(ref k, ref v) => format!("set {} {}", k, v),
            TranslationCommand::Add(ref k, ref v) => format!("add {} '{}'", k, v),
            TranslationCommand::Remove(ref k) => format!("remove {}", k),
        };
        DslToken::Command(s)
    }
}

impl GearsDsl for Translation {
    fn generate_dsl(&self) -> DslTokens {
        let mut res = DslTokens::new();

        res.push(
            TranslationCommand::Set("locale".to_string(), self.locale.clone()).as_dsl_token(),
        );

        for (_key, item) in &self.items {
            res.push(
                TranslationCommand::Add(item.key.clone(), item.value.clone()).as_dsl_token(),
            );
        }

        res
    }

    fn consume_command(&mut self, s: &str) -> Result<(), String> {
        debug!("consume_command : received command string '{:?}'", s);
        match command_grammar::translation_command(&s) {
            Ok(cmd) => {
                debug!("consume_command : received parsed command '{:?}'", cmd);
                match cmd {
                    TranslationCommand::Add(key, value) => {
                        self.items.insert(
                            key.clone(),
                            I18NString {
                                locale: self.locale.clone(),
                                key: key,
                                value: value,
                            },
                        );
                    }
                    TranslationCommand::Remove(key) => {
                        self.items.retain({
                            |k, _| k.ne(&key)
                        });
                    }
                    TranslationCommand::Set(key, value) => {
                        match key.as_ref() {
                            "locale" => {
                                self.locale = value;
                            }
                            _ => {
                                unimplemented!();
                            }
                        }
                    }
                }
                Ok(())
            }
            Err(err) => {
                error!("consume_command : {:?}", err);
                return Err(format!("{}", err));
            }
        }
    }

    fn consume_scope(&mut self, _name: &str, _tree: &Vec<DslTree>) -> Result<(), String> {
        unimplemented!();
    }
}
