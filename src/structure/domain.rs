use super::common::{Document, I18NString, Translatable, Queryable};
use structure::translation::TranslationDocument;

pub type DomainDocument = Document<Domain>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Domain {
    pub events: Events,
    pub entities: Entities,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Events {
    pub change: Vec<String>,
    pub update: Vec<String>,
    pub read: Vec<String>,
    pub delete: Vec<String>,
    pub all: Vec<String>,
}

pub type Entities = Vec<Entity>;
pub type Attributes = Vec<Attribute>;
pub type References = Vec<Reference>;

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Validation {
    pub message: I18NString,
    pub xflow: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Attribute {
    pub name: String,
    pub vtype: String,
    pub default: String,
    pub validations: Vec<Validation>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub enum ReferenceType {
    #[serde(rename = "has_many")]
    HasMany,
    #[serde(rename = "belongs_to")]
    BelongsTo,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Reference {
    pub name: String,
    pub reftype: ReferenceType,
    pub other: String,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Clone)]
pub struct Entity {
    pub name: String,
    pub attributes: Attributes,
    pub references: References,
}

impl Default for Events {
    fn default() -> Self {
        Events {
            change: Vec::<String>::new(),
            update: Vec::<String>::new(),
            read: Vec::<String>::new(),
            delete: Vec::<String>::new(),
            all: Vec::<String>::new(),
        }
    }
}

impl DomainDocument {
    pub fn has_entity(&self, name: &str) -> bool {
        match self.get_entity(name) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_entity(&self, name: &str) -> Result<&Entity, String> {
        let res: Vec<&Entity> = self.doc
            .entities
            .iter()
            .filter({
                |e| e.name.eq(name)
            })
            .collect();
        if res.len() == 1 {
            Ok(&res[0])
        } else {
            Err(format!("Entity {} does not exist", name))
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> Result<(), String> {
        if self.has_entity(&entity.name) {
            Err(format!("Entity {} already exists", entity.name))
        } else {
            self.doc.entities.push(entity);
            Ok(())
        }
    }

    pub fn remove_entity(&mut self, entity: &str) -> Result<(), String> {
        let entities = self.doc.entities.clone();

        self.doc.entities = entities
            .into_iter()
            .filter({
                |e| e.name.ne(entity)
            })
            .collect();

        Ok(())
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity {
            name: "".to_owned(),
            attributes: Attributes::new(),
            references: References::new(),
        }
    }
}

impl Entity {
    pub fn new(name: &str) -> Self {
        Entity {
            name: name.to_string().to_owned(),
            attributes: Attributes::new(),
            references: References::new(),
        }
    }

    /*
    pub fn get_attribute(self, name: &str) -> Result<&Attribute, String> {
        let res: Vec<&Attribute> = self.attributes
            .iter()
            .filter({
                |e| e.name.eq(name)
            })
            .collect();

        if res.len() == 1 {
            Ok(&res[0])
        } else {
            Err(format!("Attribute {} does not exist", name))
        }

    }
    */
}

impl Default for Domain {
    fn default() -> Self {
        Domain {
            events: Events::default(),
            entities: Entities::new(),
        }
    }
}

impl Queryable for Domain {}

impl Domain {}

impl Translatable for DomainDocument {
    fn translate_in_place(&mut self, t: &TranslationDocument) -> () {
        for entity in &mut self.doc.entities {
            for attribute in &mut entity.attributes {
                for validation in &mut attribute.validations {
                    validation.message.translate_self(&t);
                }
            }
        }
    }
    fn translate(&self, t: &TranslationDocument) -> DomainDocument {
        let mut doc = self.clone();
        doc.translate_in_place(&t);
        doc
    }

    fn all_i18n_strings(&self) -> Vec<&I18NString> {
        let mut ts = Vec::<&I18NString>::new();

        for entity in &self.doc.entities {
            for attribute in &entity.attributes {
                for validation in &attribute.validations {
                    ts.push(&validation.message);
                }
            }
        }

        ts
    }
}
