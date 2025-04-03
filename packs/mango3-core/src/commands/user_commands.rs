use uuid::Uuid;

use crate::constants::*;
use crate::enums::UserRole;
use crate::models::*;
use crate::utils::*;
use crate::CoreContext;

#[cfg(feature = "insert-user")]
use crate::enums::{Input, InputError};

#[cfg(feature = "insert-user")]
impl Validator {
    fn validate_full_name(&mut self, value: &str) -> bool {
        self.validate_presence(Input::FullName, value)
            && self.validate_length(Input::FullName, value, Some(2), Some(256))
    }

    fn validate_birthdate(&mut self, value: Option<chrono::NaiveDate>) -> bool {
        self.validate_presence(Input::Birthdate, value)
            && self.custom_validation(Input::Birthdate, InputError::IsInvalid, &|| {
                value.unwrap() <= chrono::Utc::now().date_naive()
            })
    }

    fn validate_country(&mut self, value: Option<&rust_iso3166::CountryCode>) -> bool {
        self.validate_presence(Input::CountryAlpha2, value)
    }

    fn validate_password(&mut self, input: Input, value: &str) -> bool {
        self.validate_presence(input.clone(), value) && self.validate_length(input, value, Some(6), Some(128))
    }
}

#[cfg(feature = "all-admin-users")]
pub async fn all_admin_users(core_context: &CoreContext) -> Vec<User> {
    sqlx::query_as!(
        User,
            r#"SELECT
                id,
                username,
                email,
                email_confirmed_at,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                bio,
                hashtag_ids,
                avatar_image_blob_id,
                role as "role!: UserRole",
                disabled_at,
                created_at,
                updated_at
            FROM users WHERE role IN ('admin', 'superuser')"#,
        )
        .fetch_all(&core_context.db_pool)
        .await
        .unwrap_or_default()
}


#[cfg(feature = "authenticate-user")]
pub async fn authenticate_user(core_context: &CoreContext, username_or_email: &str, password: &str) -> MutResult<User> {
    use crate::enums::Input;

    let mut validator = crate::validator!();

    validator.validate_presence(Input::UsernameOrEmail, username_or_email);
    validator.validate_presence(Input::Password, password);

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let user = get_user_by_username_or_email(core_context, username_or_email).await?;

    if verify_user_password(&user, password) {
        crate::mut_success!(user)
    } else {
        crate::mut_error!()
    }
}

#[cfg(feature = "clear-user-cache")]
pub async fn clear_user_cache(user: &User) {
    let email = user.email.to_lowercase();
    let username = user.username.to_lowercase();

    futures::join!(
        USER_BIO_HTML.cache_remove(PREFIX_USER_BIO_HTML, &user.id),
        USER_BIO_PREVIEW_HTML.cache_remove(PREFIX_USER_BIO_PREVIEW_HTML, &user.id),
        GET_USER_BY_ID.cache_remove(PREFIX_GET_USER_BY_ID, &user.id),
        GET_USER_BY_USERNAME.cache_remove(PREFIX_GET_USER_BY_USERNAME, &username),
        GET_USER_BY_USERNAME_OR_EMAIL.cache_remove(PREFIX_GET_USER_BY_USERNAME_OR_EMAIL, &username),
        GET_USER_BY_USERNAME_OR_EMAIL.cache_remove(PREFIX_GET_USER_BY_USERNAME_OR_EMAIL, &email),
    );
}

#[cfg(feature = "confirm-user-email")]
pub async fn confirm_user_email(&self, core_context: &CoreContext) -> MutResult<Self> {
    let result = sqlx::query_as!(
        User,
        r#"UPDATE users SET email_confirmed_at = current_timestamp
        WHERE disabled_at IS NULL AND email_confirmed_at IS NULL AND id = $1 RETURNING
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at"#,
        self.id, // $1
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(user) => {
            clear_user_cache(user).await;

            Ok(user)
        }
        Err(_) => Err(ValidationErrors::default()),
    }
}

#[cfg(feature = "disable-user")]
pub async fn disable_user(core_context: &CoreContext, user: &User) -> MutResult {
    let result = sqlx::query!(
        "UPDATE users SET disabled_at = current_timestamp WHERE role = 'user' AND disabled_at IS NULL AND id = $1",
        user.id
    )
    .execute(&core_context.db_pool)
    .await;

    match result {
        Ok(_) => {
            super::delete_all_user_sessions(core_context, user)
                .await
                .expect("Could not delete user sessions");

            core_context
                .jobs
                .mailer(user, crate::enums::MailerJobCommand::Disabled)
                .await;

            clear_user_cache(user).await;

            crate::mut_success!()
        }
        Err(_) => crate::mut_error!(),
    }
}

#[cfg(feature = "enable-user")]
pub async fn enable_user(core_context: &CoreContext, user: &User) -> MutResult {
    let result = sqlx::query!(
        "UPDATE users SET disabled_at = NULL WHERE disabled_at IS NOT NULL AND id = $1",
        user.id
    )
    .execute(&core_context.db_pool)
    .await;

    match result {
        Ok(_) => {
            core_context
                .jobs
                .mailer(user, crate::enums::MailerJobCommand::Enabled)
                .await;

            clear_user_cache(user).await;

            crate::mut_success!()
        }
        Err(_) => crate::mut_error!(),
    }
}

#[cfg(feature = "get-user-by-id")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ id }"#,
    ty = "cached::AsyncRedisCache<Uuid, User>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_USER_BY_ID).await } "##
)]
pub async fn get_user_by_id(core_context: &CoreContext, id: Uuid) -> sqlx::Result<User> {
    sqlx::query_as!(
        User,
        r#"SELECT
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at
        FROM users WHERE id = $1 LIMIT 1"#,
        id
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-user-by-username")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ username.to_lowercase() }"#,
    ty = "cached::AsyncRedisCache<String, User>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_USER_BY_USERNAME).await } "##
)]
pub async fn get_user_by_username(core_context: &CoreContext, username: &str) -> sqlx::Result<User> {
    if username.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as!(
        User,
        r#"SELECT
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at
        FROM users WHERE LOWER(username) = $1 LIMIT 1"#,
        username.to_lowercase()
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "get-user-by-username-or-email")]
#[cached::proc_macro::io_cached(
    map_error = r##"|_| sqlx::Error::RowNotFound"##,
    convert = r#"{ username_or_email.to_lowercase() }"#,
    ty = "cached::AsyncRedisCache<String, User>",
    create = r##" { crate::async_redis_cache!(crate::constants::PREFIX_GET_USER_BY_USERNAME_OR_EMAIL).await } "##
)]
pub async fn get_user_by_username_or_email(core_context: &CoreContext, username_or_email: &str) -> sqlx::Result<User> {
    if username_or_email.is_empty() {
        return Err(sqlx::Error::RowNotFound);
    }

    sqlx::query_as!(
        User,
        r#"SELECT
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at
        FROM users
        WHERE
            disabled_at IS NULL
            AND (LOWER(username) = $1 OR (email_confirmed_at IS NOT NULL AND LOWER(email) = $1))
        LIMIT 1"#,
        username_or_email.to_lowercase()
    )
    .fetch_one(&core_context.db_pool)
    .await
}

#[cfg(feature = "insert-user")]
pub async fn insert_user(
    core_context: &CoreContext,
    username: &str,
    email: &str,
    password: &str,
    full_name: &str,
    birthdate: &str,
    language_code: &str,
    country_alpha2: &str,
) -> MutResult<User> {
    use crate::config::USER_CONFIG;

    let mut validator = crate::validator!();

    let username = username.trim();
    let email = email.trim().to_lowercase();
    let full_name = full_name.trim();
    let birthdate = parse_date(birthdate);
    let country = find_country(country_alpha2);

    if validator.validate_presence(Input::Username, username)
        && validator.validate_length(Input::Username, username, Some(3), Some(16))
        && validator.validate_format(Input::Username, username, &REGEX_USERNAME)
        && validator.custom_validation(Input::Username, InputError::IsInvalid, &|| {
            Uuid::try_parse(username).is_err()
        })
        && validator.custom_validation(Input::Username, InputError::IsInvalid, &|| {
            !BLACKLISTED_SLUGS.contains(&username.to_lowercase().as_str())
        })
    {
        let username_exists = sqlx::query!(
            "SELECT id FROM users WHERE LOWER(username) = $1 LIMIT 1",
            username.to_lowercase() // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .is_ok();
        validator.custom_validation(Input::Username, InputError::AlreadyInUse, &|| !username_exists);
    }

    if validator.validate_presence(Input::Email, &email)
        && validator.validate_length(Input::Email, &email, Some(5), Some(256))
        && validator.validate_format(Input::Email, &email, &REGEX_EMAIL)
    {
        let email_exists = sqlx::query!(
            "SELECT id FROM users WHERE LOWER(email) = $1 LIMIT 1",
            email // $1
        )
        .fetch_one(&core_context.db_pool)
        .await
        .is_ok();
        validator.custom_validation(Input::Email, InputError::AlreadyInUse, &|| !email_exists);
    }

    validator.validate_password(Input::Password, password);

    validator.validate_full_name(full_name);

    validator.validate_birthdate(birthdate);

    validator.validate_country(country);

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let display_name = full_name.split(' ').next().unwrap();
    let encrypted_password = encrypt_password(password);

    let result = sqlx::query_as!(
        User,
        r#"INSERT INTO users (
            username,
            email,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            role,
            disabled_at
        ) VALUES (
            $1::text, $2::text, $3, $4, $5, $6, $7, $8, $9,
            (CASE WHEN $10 IS TRUE THEN current_timestamp ELSE NULL END)
        )
        RETURNING
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at"#,
        username,                                 // $1
        email,                                    // $2
        encrypted_password,                       // $3
        display_name,                             // $4
        full_name,                                // $5
        birthdate,                                // $6
        language_code,                            // $7
        country.unwrap().alpha2,                  // $8
        &USER_CONFIG.default_role() as &UserRole, // $9
        USER_CONFIG.default_disabled,             // $10
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(user) => {
            futures::future::join(
                core_context
                    .jobs
                    .admin_mailer(crate::enums::AdminMailerJobCommand::NewUser(user.clone())),
                core_context.jobs.mailer(&user, crate::enums::MailerJobCommand::Welcome),
            )
            .await;

            crate::mut_success!(user)
        }
        Err(_) => crate::mut_error!(),
    }
}

#[cfg(feature = "paginate-users")]
pub async fn paginate_users(core_context: &CoreContext, cursor_page_params: &CursorPageParams) -> CursorPage<User> {
    crate::cursor_page!(
        core_context,
        cursor_page_params,
        |node: User| node.id,
        move |core_context, after| async move { get_user_by_id(core_context, after).await.ok() },
        move |core_context, cursor_resource, limit| async move {
            let cursor_username = cursor_resource.map(|c| c.username);

            sqlx::query_as!(
                User,
                r#"SELECT
                    id,
                    username,
                    email,
                    email_confirmed_at,
                    encrypted_password,
                    display_name,
                    full_name,
                    birthdate,
                    language_code,
                    country_alpha2,
                    bio,
                    hashtag_ids,
                    avatar_image_blob_id,
                    role as "role!: UserRole",
                    disabled_at,
                    created_at,
                    updated_at
                FROM users WHERE $1::citext IS NULL OR username > $1 ORDER BY username ASC LIMIT $2"#,
                cursor_username, // $1
                limit,           // $2
            )
            .fetch_all(&core_context.db_pool)
            .await
            .unwrap_or_default()
        },
    )
    .await
}

#[cfg(feature = "reset-user-password")]
pub async fn reset_user_password(core_context: &CoreContext, user: &User, new_password: &str) -> MutResult<User> {
    let mut validator = validator!();

    validator.validate_password(Input::NewPassword, new_password);

    if !validator.is_valid {
        return crate::mut_error!(validator.errors);
    }

    let encrypted_password = encrypt_password(new_password);

    let result = sqlx::query_as!(
        Self,
        r#"UPDATE users SET encrypted_password = $2 WHERE disabled_at IS NULL AND id = $1
            RETURNING
                id,
                username,
                email,
                email_confirmed_at,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                bio,
                hashtag_ids,
                avatar_image_blob_id,
                role as "role!: UserRole",
                disabled_at,
                created_at,
                updated_at"#,
        self.id,            // $1
        encrypted_password, // $2
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(user) => {
            user.cache_remove().await;

            crate::mut_sucess!(user)
        }
        Err(_) => crate::mut_error!(),
    }
}

#[cfg(feature = "send-user-email-confirmation-code")]
pub async fn send_user_email_confirmation_code(core_context: &CoreContext, user: &User) -> MutResult<ConfirmationCode> {
    if user.email_is_confirmed() {
        return crate::mut_error!();
    }

    super::insert_confirmation_code(core_context, self, ConfirmationCodeAction::EmailConfirmation).await
}

#[cfg(feature = "send-user-login-confirmation-code")]
pub async fn send_user_login_confirmation_code(core_context: &CoreContext) -> MutResult<ConfirmationCode> {
    if !user.email_is_confirmed() {
        return crate::mut_error!();
    }

    super::insert_confirmation_code(core_context, user, ConfirmationCodeAction::LoginConfirmation).await
}

#[cfg(feature = "send-user-password-reset-code")]
pub async fn send_user_password_reset_code(core_context: &CoreContext, user: &User) -> MutResult<ConfirmationCode> {
    if !user.email_is_confirmed() {
        return crate::mut_error!();
    }

    super::insert_confirmation_code(core_context, user, crate::enums::ConfirmationCodeAction::PasswordReset).await
}

#[cfg(feature = "update-user-email")]
pub async fn update_user_email(
    core_context: &CoreContext,
    user: &User,
    email: &str,
    password: &str,
) -> MutResult<Self> {
    let email = email.trim().to_lowercase();

    let mut validator = validator!();

    if validator.validate_presence(Input::Email, &email)
        && validator.validate_length(Input::Email, &email, Some(5), Some(256))
        && validator.validate_format(Input::Email, &email, &REGEX_EMAIL)
    {
        let email_exists = query!(
            "SELECT id FROM users WHERE id != $1 AND LOWER(email) = $2 LIMIT 1",
            self.id, // $1
            email,   // $2
        )
        .fetch_one(&core_context.db_pool)
        .await
        .is_ok();
        validator.custom_validation(Input::Email, InputError::AlreadyInUse, &|| !email_exists);
    }

    if validator.validate_presence(Input::Password, password) {
        validator.custom_validation(Input::Password, InputError::IsInvalid, &|| {
            self.verify_password(password)
        });
    }

    if !validator.is_valid {
        return Err(validator.errors);
    }

    if self.email == email {
        return Ok(self.clone());
    }

    let result = query_as!(
        Self,
        r#"UPDATE users SET email = $2::text, email_confirmed_at = NULL WHERE disabled_at IS NULL AND id = $1
        RETURNING
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at"#,
        self.id, // $1
        email,   // $2
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(user) => {
            user.cache_remove().await;

            Ok(user)
        }
        Err(_) => Err(ValidationErrors::default()),
    }
}

#[cfg(feature = "update-user-password")]
pub async fn update_user_password(
    core_context: &CoreContext,
    user: &User,
    current_password: &str,
    new_password: &str,
) -> MutResult<User> {
    let mut validator = Validator::default();

    if validator.validate_presence(Input::CurrentPassword, current_password) {
        validator.custom_validation(Input::CurrentPassword, InputError::IsInvalid, &|| {
            self.verify_password(current_password)
        });
    }

    validator.validate_password(Input::NewPassword, new_password);

    if !validator.is_valid {
        return Err(validator.errors);
    }

    if self.verify_password(new_password) {
        return Ok(self.clone());
    }

    let encrypted_password = encrypt_password(new_password);

    let result = query_as!(
        Self,
        r#"UPDATE users SET encrypted_password = $2 WHERE disabled_at IS NULL AND id = $1 RETURNING
            id,
            username,
            email,
            email_confirmed_at,
            encrypted_password,
            display_name,
            full_name,
            birthdate,
            language_code,
            country_alpha2,
            bio,
            hashtag_ids,
            avatar_image_blob_id,
            role as "role!: UserRole",
            disabled_at,
            created_at,
            updated_at"#,
        self.id,            // $1
        encrypted_password, // $2
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(user) => {
            user.cache_remove().await;

            Ok(user)
        }
        Err(_) => Err(ValidationErrors::default()),
    }
}

#[cfg(feature = "update-user-profile)]
pub async fn update_user_profile(
        core_context: &CoreContext,
        user: &User,
        display_name: &str,
        full_name: &str,
        birthdate: &str,
        country_alpha2: &str,
        bio: &str,
        avatar_image_blob: Option<&Blob>,
    ) -> Result<User, ValidationErrors> {
        let mut validator = crate::validator!();

        let display_name = display_name.trim();
        let full_name = full_name.trim();
        let birthdate = parse_date(birthdate);
        let country = find_country(country_alpha2);
        let bio = bio.trim();
        let avatar_image_blob_id = avatar_image_blob.map(|blob| blob.id);

        if validator.validate_presence(Input::DisplayName, display_name) {
            validator.validate_length(Input::DisplayName, display_name, Some(2), Some(256));
        }

        validator.validate_full_name(full_name);

        validator.validate_birthdate(birthdate);

        validator.validate_country(country);

        validator.validate_length(Input::Bio, bio, None, Some(1024));

        if !validator.is_valid {
            return crate::mut_error!(validator.errors);
        }

        let hashtags = get_or"_insert_many_hashtags(core_context, bio).await?;
        let hashtag_ids = hashtags.iter().map(|hashtag| hashtag.id).collect::<Vec<Uuid>>();

        let result = sqlx::query_as!(
            User,
            r#"UPDATE users
            SET
                display_name = $2,
                full_name = $3,
                birthdate = $4,
                country_alpha2 = $5,
                bio = $6,
                hashtag_ids = $7,
                avatar_image_blob_id = $8
            WHERE disabled_at IS NULL AND id = $1 RETURNING
                id,
                username,
                email,
                email_confirmed_at,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                bio,
                hashtag_ids,
                avatar_image_blob_id,
                role as "role!: UserRole",
                disabled_at,
                created_at,
                updated_at"#,
            self.id,                 // $1
            display_name,            // $2
            full_name,               // $3
            birthdate,               // $4
            country.unwrap().alpha2, // $5
            bio,                     // $6
            &hashtag_ids,            // $7
            avatar_image_blob_id,    // $8
        )
        .fetch_one(&core_context.db_pool)
        .await;

        match result {
            Ok(user) => {
                clear_user_cache(user).await;

                crate::mut_success!(user)
            }
            Err(_) => crate::mut_error!(),
    }
}


#[cfg(feature = "update-user-role")]
pub async fn update_user_role(core_context: &CoreContext, user: &User, role: UserRole) -> MutResult<User> {
    if role == UserRole::Superuser {
        let _ = sqlx::query!(
            r#"UPDATE users SET role = 'admin' WHERE role = 'superuser' AND id != $1"#,
            user.id
        )
        .execute(&core_context.db_pool)
        .await;
    }

    let result = sqlx::query_as!(
        User,
        r#"UPDATE users SET role = $2 WHERE id = $1 RETURNING
                id,
                username,
                email,
                email_confirmed_at,
                encrypted_password,
                display_name,
                full_name,
                birthdate,
                language_code,
                country_alpha2,
                bio,
                hashtag_ids,
                avatar_image_blob_id,
                role as "role!: UserRole",
                disabled_at,
                created_at,
                updated_at"#,
        user.id,          // $1
        role as UserRole, // $2
    )
    .fetch_one(&core_context.db_pool)
    .await;

    match result {
        Ok(user) => {
            clear_user_cache(&user).await;

            crate::mut_success!(user)
        }
        Err(_) => crate::mut_error!(),
    }
}

#[cfg(feature = "verify-user-password")]
pub fn verify_user_password(user: &User, password: &str) -> bool {
    if user.encrypted_password.is_empty() {
        return false;
    }

    verify_password(password, &user.encrypted_password)
}

#[cfg(test)]
mod tests {
    use crate::test_utils::{fake_username, fake_uuid, insert_test_user, setup_core_context};

    use super::{
        disable_user, get_user_by_id, get_user_by_username, get_user_by_username_or_email, update_user_role, UserRole,
    };

    #[tokio::test]
    async fn should_disable_user() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = disable_user(&core_context, &user).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_get_by_id() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = get_user_by_id(&core_context, user.id).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_by_id_when_is_invalid() {
        let core_context = setup_core_context().await;
        let id = fake_uuid();

        let result = get_user_by_id(&core_context, id).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_by_username() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = get_user_by_username(&core_context, &user.username).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_by_username_when_is_invalid() {
        let core_context = setup_core_context().await;
        let username = fake_username();

        let result = get_user_by_username(&core_context, &username).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_by_username_or_email() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = get_user_by_username_or_email(&core_context, &user.username).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn should_not_get_by_email_when_is_unverified() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = get_user_by_username_or_email(&core_context, &user.email).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn should_get_some_users() {
        let core_context = setup_core_context().await;

        insert_test_user(&core_context).await;

        let cursor_page = User::paginate_by_username_asc(&core_context, &CursorPageParams::default()).await;

        assert!(!cursor_page.nodes.is_empty());
    }

    #[tokio::test]
    async fn should_update_user_role() {
        let core_context = setup_core_context().await;
        let user = insert_test_user(&core_context).await;

        let result = update_user_role(&core_context, &user, UserRole::Admin).await;

        assert!(result.is_ok());

        let user = result.unwrap();

        assert_eq!(user.role, UserRole::Admin);
    }
}
