# Mango³

<p align="center">
  <img alt="icon" src="https://raw.githubusercontent.com/mangocubed/mango3/refs/heads/main/assets/icon.svg"/>
</p>

A free and open source website builder and content management system platform written in Rust.

[![release](https://img.shields.io/github/v/release/mangocubed/mango3.svg?include_prereleases)](https://github.com/mangocubed/mango3/releases/latest)
[![ci](https://github.com/mangocubed/mango3/actions/workflows/ci.yaml/badge.svg)](https://github.com/mangocubed/mango3/actions/workflows/ci.yaml)
[![dependencies](https://deps.rs/repo/github/mangocubed/mango3/status.svg)](https://deps.rs/repo/github/mangocubed/mango3)

[Website](https://mango3.app) | [Source code](https://github.com/mangocubed/mango3)

<p align="center">
  <img alt="screenshot" src="https://raw.githubusercontent.com/mangocubed/mango3/refs/heads/main/assets/screenshot.png"/>
</p>

## Deployment

> [!WARNING]
> This platform is still in an early stage, so I wouldn't recommend it for production use yet.

### Requirements

- **Rust** 1.82+
- **PostgreSQL** 16+
- **Redis** 7+
- **Node.js** 20+
- **Nginx** 1.26+ (or any reverse proxy)
- **Dnsmasq** 2.91+ (or any domain name server)

### Environment variables

| Name                            | Type    | Default                                                          |
| ------------------------------- | ------- | ---------------------------------------------------------------- |
| BASIC_ABOUT_URL                 | String  |                                                                  |
| BASIC_COPYRIGHT                 | String  | © 2025, Mango³ Team                                             |
| BASIC_DESCRIPTION               | String  | An open source content management system platform.               |
| BASIC_DOMAIN                    | String  | mango3.local                                                     |
| BASIC_ENABLE_REGISTER           | String  | true                                                             |
| BASIC_PRIVACY_POLICY_URL        | String  |                                                                  |
| BASIC_SECURE                    | Boolean | false                                                            |
| BASIC_SUPPORT_EMAIL_ADDRESS     | String  | support@mango3.local                                             |
| BASIC_TERMS_OF_SERVICE_URL      | String  |                                                                  |
| BASIC_TITLE                     | String  | Mango³ Dev                                                       |
| CACHE_REDIS_URL                 | String  | redis://127.0.0.1:6379/2                                         |
| CACHE_TTL                       | String  | 3600                                                             |
| DATABASE_MAX_CONNECTIONS        | Integer | 5                                                                |
| DATABASE_URL                    | String  | postgres://mango3:mango3@127.0.0.1:5432/mango3_dev               |
| JOBS_REDIS_URL                  | String  | redis://127.0.0.1:6379/0                                         |
| MAILER_ENABLE                   | Boolean | false                                                            |
| MAILER_SENDER_ADDRESS           | String  | Mango³ Dev <no-reply@mango3.local>                               |
| MAILER_SMTP_ADDRESS             | String  | mango3.local                                                     |
| MAILER_SMTP_PASSWORD            | String  |                                                                  |
| MAILER_SMTP_SECURITY            | String  | none                                                             |
| MAILER_SMTP_USERNAME            | String  |                                                                  |
| MISC_CLIENT_IP_SOURCE           | String  | XRealIp                                                          |
| MISC_CONFIRMATION_CODE_LENGTH   | Integer | 6                                                                |
| MISC_FONT_PATH                  | String  | /usr/share/fonts/truetype/dejavu/DejaVuSans.ttf                  |
| MISC_IMAGE_OPS_FILTER_TYPE      | String  | CatmullRom                                                       |
| MISC_INVITATION_CODE_LENGTH     | Integer | 6                                                                |
| MISC_MAX_COMMENT_CONTENT_LENGTH | Integer | 8192                                                             |
| MISC_MAX_POST_CONTENT_LENGTH    | Integer | 16384                                                            |
| MISC_STORAGE_PATH               | String  | ./storage                                                        |
| SESSIONS_KEY                    | String  | abcdefghijklmnopqrestuvvwxyz0123456789ABCDEFGHIJKLMNOPQRESTUVVWX |
| SESSIONS_REDIS_URL              | String  | redis://127.0.0.1:6379/1                                         |
| USER_DEFAULT_DISABLED           | Boolean | false                                                            |
| USER_DEFAULT_ROLE               | String  | user                                                             |
| WEBSITE_MAX_STORAGE             | Integer | 1 GiB                                                            |

### Installation and setup

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

> [!TIP]
> You should also use the files located in `config/` to configure Nginx and Dnsmasq.

### Running the services

Run monitor:

```sh
cargo run --bin mango3-monitor
```

Run uploads:

```sh
cargo run --bin mango3-uploads
```

Run web applications:

- Home:

```sh
cargo leptos serve --project mango3-home
```

- Accounts:

```sh
cargo leptos serve --project mango3-accounts
```

- Admin:

```sh
cargo leptos serve --project mango3-admin
```

- My account:

```sh
cargo leptos serve --project mango3-my-account
```

- Studio:

```sh
cargo leptos serve --project mango3-studio
```

- Websites:

```sh
cargo leptos serve --project mango3-websites
```
