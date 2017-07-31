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
pub type Validations = Vec<Validation>;

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

impl Attribute {
    pub fn new(name: &str, attr_type: &str) -> Self {
        Attribute {
            name: name.to_string().clone(),
            vtype: attr_type.to_string().clone(),
            default: "".to_string(),
            validations: Validations::new(),
        }
    }
}

impl Domain {
    pub fn has_entity(&mut self, name: &str) -> bool {
        match self.get_entity(name) {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    pub fn get_entity(&mut self, name: &str) -> Result<&Entity, String> {
        let mut res: Vec<&Entity> = self.entities
            .iter()
            .filter({
                |e| e.name.eq(name)
            })
            .collect();
        if res.len() == 1 {
            Ok(&mut res[0])
        } else {
            Err(format!("Entity {} does not exist", name))
        }
    }

    pub fn add_entity(&mut self, entity: Entity) -> Result<(), String> {
        if self.has_entity(&entity.name) {
            Err(format!("Entity {} already exists", entity.name))
        } else {
            self.entities.push(entity);
            Ok(())
        }
    }

    pub fn remove_entity(&mut self, entity: &str) -> Result<(), String> {
        let entities = self.entities.clone();

        self.entities = entities
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

    pub fn get_attribute(self, name: &str) -> Result<Attribute, String> {
        let res: Vec<&Attribute> = self.attributes
            .iter()
            .filter({
                |e| e.name.eq(name)
            })
            .collect();
        if res.len() == 1 {
            Ok(res[0].clone())
        } else {
            Err(format!("Attribute {} does not exist", name))
        }
    }

    pub fn add_attribute(&mut self, attr: Attribute) -> Result<(), String> {
        self.attributes.push(attr);
        Ok(())
    }
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

use dsl::command::{GearsDsl, DslItem, command_grammar};

#[derive(Debug, Eq, PartialEq)]
pub enum DomainCommand {
    AddEntity(String),
    RemoveEntity(String),
    AddAttribute(String, String),
    RemoveAttribute(String),
    AddValidation(String, String),
    RemoveValidation(String),
}

impl DomainCommand {
    fn as_dsl_item(&self) -> DslItem {
        let s = match *self {
            DomainCommand::AddEntity(ref e) => format!("add entity {}", e),
            DomainCommand::RemoveEntity(ref e) => format!("remove entity {}", e),
            DomainCommand::AddAttribute(ref a, ref t) => format!("add attribute {}:{}", a, t),
            DomainCommand::RemoveAttribute(ref a) => format!("remove attribute {}", a),
            DomainCommand::AddValidation(ref v, ref t) => format!("add validation {} '{}'", v, t),
            DomainCommand::RemoveValidation(ref v) => format!("remove validation {}", v),
        };
        DslItem::Command(s)
    }
}

struct DomainDslState {
    indent: usize,
    entity: String,
    attribute: String,
}

impl Default for DomainDslState {
    fn default() -> Self {
        DomainDslState {
            indent: 0,
            entity: "".to_owned(),
            attribute: "".to_owned(),
        }
    }
}

impl GearsDsl for Domain {
    fn generate_dsl(&self) -> Vec<DslItem> {
        let mut res = Vec::<DslItem>::new();

        for entity in &self.entities {
            res.push(DomainCommand::AddEntity(entity.name.clone()).as_dsl_item());

            if entity.attributes.len() > 0 {
                res.push(DslItem::With(entity.name.clone()));
                res.push(DslItem::BlockOpen);

                for attribute in &entity.attributes {
                    res.push(
                        DomainCommand::AddAttribute(attribute.name.clone(), attribute.vtype.clone())
                            .as_dsl_item(),
                    );

                    if attribute.validations.len() > 0 {
                        res.push(DslItem::With(attribute.name.clone()));

                        res.push(DslItem::BlockOpen);
                        for validation in &attribute.validations {
                            res.push(
                                DomainCommand::AddValidation(
                                    validation.xflow.clone(),
                                    validation.message.value.clone(),
                                ).as_dsl_item(),
                            );
                        }
                        res.push(DslItem::BlockClose);
                    }
                }
                res.push(DslItem::BlockClose);
            }
        }
        res
    }

    fn consume_dsl(&mut self, items: &Vec<DslItem>) -> Result<(), String> {
        let mut state = DomainDslState::default();

        for item in items {
            match *item {
                DslItem::Comment(_) => {}
                DslItem::With(ref s) => {
                    match state.indent {
                        0 => {
                            state.entity = s.clone();
                        }
                        1 => {
                            state.attribute = s.clone();
                        }
                        _ => {
                            error!("consume_dsl: too deeply nested!");
                            return Err("consume_dsl: too deeply nested".to_owned());
                        }
                    }
                }
                DslItem::BlockOpen => {
                    state.indent += 1;
                }
                DslItem::BlockClose => {
                    state.indent += 1;
                }
                DslItem::Command(ref s) => {
                    match command_grammar::domain_command(&s) {
                        Ok(cmd) => {
                            match cmd {
                                DomainCommand::AddEntity(e) => {
                                    if state.indent != 0 {
                                        return Err("Bad indent for Entity".to_owned());
                                    } else {
                                        self.add_entity(Entity::new(&e));
                                    }
                                }
                                DomainCommand::RemoveEntity(e) => {
                                    if state.indent != 0 {
                                        return Err("Bad indent for Entity".to_owned());
                                    } else {
                                        self.remove_entity(&e);
                                    }
                                }
                                DomainCommand::AddAttribute(attr, attr_type) => {
                                    if state.indent != 1 {
                                        return Err("Bad indent for Attribute".to_owned());
                                    } else {
                                        info!("Domain DSL");
                                        /*
                                        match self.get_entity(&state.entity) {
                                            Ok(e) => {
                                                e.add_attribute(
                                                    Attribute::new(&attr, &attr_type),
                                                );
                                            }
                                            Err(err) => return Err(err),
                                        }
                                        */
                                    }
                                }
                                DomainCommand::RemoveAttribute(attr) => {
                                    info!("Domain DSL");
                                }
                                DomainCommand::AddValidation(val, val_msg) => {
                                    info!("Domain DSL");
                                }
                                DomainCommand::RemoveValidation(val) => {
                                    info!("Domain DSL");
                                }
                            }
                        }
                        Err(err) => return Err(format!("Parsing error domain_command : {:?}", err)),
                    }
                }
            }
        }

        Ok(())
    }
}
