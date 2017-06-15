pub type DomainDocument = super::common::Document<Domain>;

#[derive(Debug, Serialize, Clone)]
pub struct Domain {
    pub events: Events,
    pub entities: Entities,
}

#[derive(Debug, Serialize, Clone)]
pub struct Events {
    pub change: Vec<String>,
    pub update: Vec<String>,
    pub read: Vec<String>,
    pub delete: Vec<String>,
    pub all: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Attribute {
    pub name: String,
    pub vtype: String,
    pub access: String,
    pub storage: String,
    pub default: String,
    pub validations: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Reference {
    pub name: String,
    pub rtype: String,
    pub ltype: String,
    pub target: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct Entity {
    pub name: String,
    pub attributes: Attributes,
}

pub type Entities = Vec<Entity>;
pub type Attributes = Vec<Attribute>;

impl Domain {}
