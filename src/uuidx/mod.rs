use uuid::Uuid;

pub fn generate_new_v4() -> Uuid {
    Uuid::new_v4()
}
