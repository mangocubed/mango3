use crate::enums::{ConfirmationCodeAction, Input};
use crate::models::ConfirmationCode;
use crate::validator::{ValidationErrors, Validator, ValidatorTrait};
use crate::CoreContext;

use super::User;

impl User {
    pub async fn authenticate(
        core_context: &CoreContext,
        username_or_email: &str,
        password: &str,
    ) -> Result<Self, ValidationErrors> {
        let mut validator = Validator::default();

        validator.validate_presence(Input::UsernameOrEmail, username_or_email);
        validator.validate_presence(Input::Password, password);

        if !validator.is_valid {
            return Err(validator.errors);
        }

        let user = Self::get_by_username_or_email(core_context, username_or_email)
            .await
            .map_err(|_| ValidationErrors::default())?;

        if user.verify_password(password) {
            Ok(user)
        } else {
            Err(ValidationErrors::default())
        }
    }

    pub async fn send_login_confirmation_code(
        &self,
        core_context: &CoreContext,
    ) -> Result<ConfirmationCode, ValidationErrors> {
        if !self.email_is_confirmed() {
            return Err(ValidationErrors::default());
        }

        ConfirmationCode::insert(core_context, self, ConfirmationCodeAction::LoginConfirmation)
            .await
            .map_err(|_| ValidationErrors::default())
    }
}
