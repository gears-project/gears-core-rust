use uuid::Uuid;

pub fn uuid_to_label(id: &Uuid) -> String {
    id.simple().to_string()
}
