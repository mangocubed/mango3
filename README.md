# Mango³

## Requirements

- Rust
- PostgreSQL
- Node.js

## Environment variables

| Name                     | Type    | Default                                            |
| ------------------------ | ------- | -------------------------------------------------- |
| BASIC_COPYRIGHT          | String  | © 2024, Mango³ Team                                |
| BASIC_TITLE              | String  | Mango³ Dev                                         |
| DATABASE_MAX_CONNECTIONS | Integer | 5                                                  |
| DATABASE_URL             | String  | postgres://mango3:mango3@127.0.0.1:5432/mango3_dev |
| JOBS_REDIS_URL           | String  | redis://127.0.0.1:6379/0                           |
| MAILER_ENABLE            | Boolean | false                                              |
| MAILER_SENDER_ADDRESS    | String  | Mango³ Dev <no-reply@localhost>                    |
| MAILER_SMTP_ADDRESS      | String  | localhost                                          |
| MAILER_SMTP_PASSWORD     | String  |                                                    |
| MAILER_SMTP_SECURITY     | String  | none                                               |
| MAILER_SMTP_USERNAME     | String  |                                                    |

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

Run web applications:

* Home:

```sh
cargo run --project mango3-home
```