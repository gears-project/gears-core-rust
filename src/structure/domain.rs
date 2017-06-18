use super::common::Document;

pub type DomainDocument = Document<Domain>;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Domain {
    pub events: Events,
    pub entities: Entities,
}

impl Default for Domain {
    fn default() -> Self {
        Domain {
            events: Events::default(),
            entities: Entities::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Events {
    pub change: Vec<String>,
    pub update: Vec<String>,
    pub read: Vec<String>,
    pub delete: Vec<String>,
    pub all: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub vtype: String,
    pub default: String,
    pub validations: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReferenceType {
    #[serde(rename = "has_many")]
    HasMany,
    #[serde(rename = "belongs_to")]
    BelongsTo,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reference {
    pub name: String,
    pub reftype: ReferenceType,
    pub other: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity {
    pub name: String,
    pub attributes: Attributes,
    pub references: References,
}

pub type Entities = Vec<Entity>;
pub type Attributes = Vec<Attribute>;
pub type References = Vec<Reference>;

impl Domain {}
