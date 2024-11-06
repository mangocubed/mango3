use super::User;

use crate::models::verify_password;

impl User {
    pub fn verify_password(&self, password: &str) -> bool {
        if self.encrypted_password.is_empty() {
            return false;
        }

        verify_password(password, &self.encrypted_password)
    }
}
