# Mango³

## Requirements

- Rust
- PostgreSQL
- Node.js

## Environment variables

| Name                          | Type    | Default                                                          |
| ----------------------------- | ------- | ---------------------------------------------------------------- |
| BASIC_COPYRIGHT               | String  | © 2024, Mango³ Team                                              |
| BASIC_DOMAIN                  | String  | mango3.localhost                                                 |
| BASIC_ENABLE_REGISTER         | String  | true                                                             |
| BASIC_SECURE                  | Boolean | false                                                            |
| BASIC_TITLE                   | String  | Mango³ Dev                                                       |
| DATABASE_MAX_CONNECTIONS      | Integer | 5                                                                |
| DATABASE_URL                  | String  | postgres://mango3:mango3@127.0.0.1:5432/mango3_dev               |
| JOBS_REDIS_URL                | String  | redis://127.0.0.1:6379/0                                         |
| MAILER_ENABLE                 | Boolean | false                                                            |
| MAILER_SENDER_ADDRESS         | String  | Mango³ Dev <no-reply@localhost>                                  |
| MAILER_SMTP_ADDRESS           | String  | localhost                                                        |
| MAILER_SMTP_PASSWORD          | String  |                                                                  |
| MAILER_SMTP_SECURITY          | String  | none                                                             |
| MAILER_SMTP_USERNAME          | String  |                                                                  |
| MISC_CONFIRMATION_CODE_LENGTH | Integer | 6                                                                |
| MISC_INVITATION_CODE_LENGTH   | Integer | 6                                                                |
| MISC_STORAGE_PATH             | String  | ./storage                                                        |
| SESSIONS_KEY                  | String  | abcdefghijklmnopqrestuvvwxyz0123456789ABCDEFGHIJKLMNOPQRESTUVVWX |
| SESSIONS_REDIS_URL            | String  | redis://127.0.0.1:6379/1                                         |

## Installation

Install dependencies:

```sh
cargo install sqlx-cli
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
