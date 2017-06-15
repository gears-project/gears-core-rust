#[derive(Debug, Serialize, Clone)]
pub struct Document<T> {
    pub id: i64,
    pub doctype: String,
    pub doctype_version: i64,
    pub version: i64,
    pub name: String,
    pub doc: T,
}
