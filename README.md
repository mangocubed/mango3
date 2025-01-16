# Mango³

## Requirements

- Rust
- PostgreSQL
- Node.js
- Nginx (or any reverse proxy)

## Environment variables

| Name                          | Type    | Default                                                          |
| ----------------------------- | ------- | ---------------------------------------------------------------- |
| BASIC_ABOUT_URL               | String  |                                                                  |
| BASIC_COPYRIGHT               | String  | © 2025, Mango³ Team                                              |
| BASIC_DOMAIN                  | String  | mango3.local                                                     |
| BASIC_ENABLE_REGISTER         | String  | true                                                             |
| BASIC_GOOGLE_ADSENSE_CLIENT   | String  |                                                                  |
| BASIC_PRIVACY_POLICY_URL      | String  |                                                                  |
| BASIC_SECURE                  | Boolean | false                                                            |
| BASIC_TERMS_OF_SERVICE_URL    | String  |                                                                  |
| BASIC_TITLE                   | String  | Mango³ Dev                                                       |
| DATABASE_MAX_CONNECTIONS      | Integer | 5                                                                |
| DATABASE_URL                  | String  | postgres://mango3:mango3@127.0.0.1:5432/mango3_dev               |
| JOBS_REDIS_URL                | String  | redis://127.0.0.1:6379/0                                         |
| MAILER_ENABLE                 | Boolean | false                                                            |
| MAILER_SENDER_ADDRESS         | String  | Mango³ Dev <no-reply@mango3.local>                               |
| MAILER_SMTP_ADDRESS           | String  | mango3.local                                                     |
| MAILER_SMTP_PASSWORD          | String  |                                                                  |
| MAILER_SMTP_SECURITY          | String  | none                                                             |
| MAILER_SMTP_USERNAME          | String  |                                                                  |
| MISC_CLIENT_IP_SOURCE         | String  | XRealIp                                                          |
| MISC_CONFIRMATION_CODE_LENGTH | Integer | 6                                                                |
| MISC_FONT_PATH                | String  | /usr/share/fonts/truetype/dejavu/DejaVuSans.ttf                  |
| MISC_DEFAULT_USER_ROLE        | String  | user                                                             |
| MISC_INVITATION_CODE_LENGTH   | Integer | 6                                                                |
| MISC_MAX_POST_CONTENT_LENGTH  | Integer | 16384                                                            |
| MISC_STORAGE_PATH             | String  | ./storage                                                        |
| SESSIONS_KEY                  | String  | abcdefghijklmnopqrestuvvwxyz0123456789ABCDEFGHIJKLMNOPQRESTUVVWX |
| SESSIONS_REDIS_URL            | String  | redis://127.0.0.1:6379/1                                         |

## Installation

Install dependencies:

```sh
cargo install sqlx-cli --no-default-features --features completions,native-tls,postgres
cargo install cargo-leptos@0.2.24
npm install
```

Setup database:

```sh
sqlx database setup
```

Run database migrations:

```sh
sqlx migrate run
```

## Deployment

Run monitor:

```sh
cargo run --bin mango3-monitor
```

Run uploads:

```sh
cargo run --bin mango3-uploads
```

Run web applications:

* Home:

```sh
cargo leptos serve --project mango3-home
```

* Accounts:

```sh
cargo leptos serve --project mango3-accounts
```

* My account:

```sh
cargo leptos serve --project mango3-my-account
```

* Studio:

```sh
cargo leptos serve --project mango3-studio
```

* Websites:

```sh
cargo leptos serve --project mango3-websites
```
