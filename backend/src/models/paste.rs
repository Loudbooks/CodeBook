use serde::{Deserialize, Serialize};
use crate::models::user::UserDTO;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Paste {
    pub id: String,
    pub created: u64,
    pub creator_ip: String,
    pub expires_at: u64,
    pub language: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasteDTO {
    pub id: String,
    pub user: UserDTO,
    pub created: u64,
    pub expires_at: u64,
    pub language: String
}

impl Paste {
    pub fn to_public_dto(&self, user: UserDTO) -> PasteDTO {
        PasteDTO {
            id: self.id.clone(),
            user,
            created: self.created,
            expires_at: self.expires_at,
            language: self.language.clone()
        }
    }
}
