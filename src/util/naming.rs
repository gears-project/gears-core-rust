use uuid::Uuid;

pub fn uuid_to_label(id: &Uuid) -> String {
    id.to_simple().to_string()
}

pub fn label_to_uuid(id: &str) -> Result<Uuid, String> {
    match Uuid::parse_str(id) {
        Ok(id) => Ok(id),
        Err(_) => Err("Invalid uuid form".to_owned())
    }
}
