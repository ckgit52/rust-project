use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PasswordChangeRequest {
    pub username: String,
    pub old_password: String,
    pub new_password: String,
}
