use uuid::Uuid;

pub mod pwd;

pub struct User {
    id: Uuid,
    handle: String,
}

impl User {
    pub fn is_infradmin(&self) -> bool {
        self.id.is_max()
    }
}
