# Changelog

## [0.0.0-alpha.13] - 2025-03-02

### Changed

- Bump chrono from 0.4.39 to 0.4.40. (https://github.com/mangocubed/mango3/pull/255)
- Bump clap from 4.5.30 to 4.5.31. (https://github.com/mangocubed/mango3/pull/257)
- Bump daisyui from 5.0.0-beta.8 to 5.0.0. (https://github.com/mangocubed/mango3/pull/256, https://github.com/mangocubed/mango3/pull/258)
- Bump tailwindcss from 4.0.8 to 4.0.9. (https://github.com/mangocubed/mango3/pull/250)
- Bump leptos_i18n from 0.5.6 to 0.5.8. (https://github.com/mangocubed/mango3/pull/259)
- Bump serde_json from 1.0.138 to 1.0.139. (https://github.com/mangocubed/mango3/pull/260)
- Bump uuid from 1.13.1 to 1.15.1. (https://github.com/mangocubed/mango3/pull/261)
- Bump fred from 10.0.4 to 10.1.0. (https://github.com/mangocubed/mango3/pull/262)
- Bump attohttpc from 0.28.5 to 0.29.1. (https://github.com/mangocubed/mango3/pull/263)

### Fixed

- Add collapse-open to safeList in tailwind.config.js. (https://github.com/mangocubed/mango3/pull/264)

## [0.0.0-alpha.12] - 2025-02-26

### Added

- Cargo features for blobs in mango3-core. (https://github.com/mangocubed/mango3/pull/247)
- Cargo features for content. (https://github.com/mangocubed/mango3/pull/249)

### Changed

- Bump strum from 0.27.0 to 0.27.1. (https://github.com/mangocubed/mango3/pull/245)
- Bump tailwindcss from 4.0.7 to 4.0.8. (https://github.com/mangocubed/mango3/pull/241)
- Bump fake from 3.1.0 to 4.0.0. (https://github.com/mangocubed/mango3/pull/246)
- Bump cargo-leptos from 0.2.28 to 0.2.29. (https://github.com/mangocubed/mango3/pull/251)
- Use LocalResource to get websites in studio selector. (https://github.com/mangocubed/mango3/pull/252)
- Use LocalResource to get post reactions. (https://github.com/mangocubed/mango3/pull/253)

## [0.0.0-alpha.11] - 2025-02-24

### Added

- Portuguese translations. (https://github.com/mangocubed/mango3/pull/236)
- Show alert when email is unconfirmed. (https://github.com/mangocubed/mango3/pull/237)
- 2FA with email. (https://github.com/mangocubed/mango3/pull/238)
- Delete expired confirmation codes. (https://github.com/mangocubed/mango3/pull/239)

### Changed

- Bump serde from 1.0.217 to 1.0.218. (https://github.com/mangocubed/mango3/pull/235)
- Bump lettre from 0.11.13 to 0.11.14. (https://github.com/mangocubed/mango3/pull/240)
- Bump @types/node from 22.13.4 to 22.13.5. (https://github.com/mangocubed/mango3/pull/242)
- Refactor confirmation codes. (https://github.com/mangocubed/mango3/pull/243)

### Fixed

- Prevent submitting post when closing preview modal. (https://github.com/mangocubed/mango3/pull/234)

## [0.0.0-alpha.10] - 2025-02-20

### Added

- Modal component. (https://github.com/mangocubed/mango3/pull/232)

### Changed

- Bump lettre from 0.11.12 to 0.11.13. (https://github.com/mangocubed/mango3/pull/229)
- Bump tailwindcss from 4.0.6 to 4.0.7. (https://github.com/mangocubed/mango3/pull/230)

### Fixed

- TopBar responsiveness with daisyUI v5. (https://github.com/mangocubed/mango3/pull/231)

## [0.0.0-alpha.9] - 2025-02-19

### Fixed

- Version in Cargo.toml was wrong. (https://github.com/mangocubed/mango3/pull/228)

## [0.0.0-alpha.8] - 2025-02-19

### Added

- Store navigation items query in a cache. (https://github.com/mangocubed/mango3/pull/220)
- Disable or enable users in admin panel. (https://github.com/mangocubed/mango3/pull/223)
- Show labels when users is disabled or is admin. (https://github.com/mangocubed/mango3/pull/224)
- Add env var to disable users by default. (https://github.com/mangocubed/mango3/pull/225)
- Notify admins by email when a user account is created. (https://github.com/mangocubed/mango3/pull/226)

### Changed

- Bump cargo-leptos from 0.2.26 to 0.2.28 in GH Actions. (https://github.com/mangocubed/mango3/pull/217)
- Bump @types/node from 22.13.1 to 22.13.4. (https://github.com/mangocubed/mango3/pull/218)
- Bump attohttpc from 0.28.2 to 0.28.5. (https://github.com/mangocubed/mango3/pull/219)
- Bump handlebars from 6.3.0 to 6.3.1. (https://github.com/mangocubed/mango3/pull/221)
- Bump clap from 4.5.29 to 4.5.30. (https://github.com/mangocubed/mango3/pull/222)
- Bump tailwindcss from 3.4.17 to 4.0.6. (https://github.com/mangocubed/mango3/pull/206)

### Fixed

- Move async_t_string! outside closure in websites. (https://github.com/mangocubed/mango3/pull/216)

## [0.0.0-alpha.7] - 2025-02-13

### Added

- Admin panel. (https://github.com/mangocubed/mango3/pull/208)
- Enable feature dynamic_load in leptos_i18n. (https://github.com/mangocubed/mango3/pull/211, https://github.com/mangocubed/mango3/pull/214)

### Changed

- Bump leptos-use from 0.15.5 to 0.15.6. (https://github.com/mangocubed/mango3/pull/205)
- Bump strum from 0.26.3 to 0.27.0. (https://github.com/mangocubed/mango3/pull/207)
- Bump clap from 4.5.28 to 4.5.29. (https://github.com/mangocubed/mango3/pull/209)
- Bump leptos from 0.7.5 to 0.7.7. (https://github.com/mangocubed/mango3/pull/210)
- Bump @faker-js/faker from 9.4.0 to 9.5.0. (https://github.com/mangocubed/mango3/pull/212)
- Bump pulldown-cmark from 0.12.2 to 0.13.0. (https://github.com/mangocubed/mango3/pull/213)

### Fixed

- Get or initialize AsyncRedisCache before remove. (https://github.com/mangocubed/mango3/pull/202)
- Make request helpers return text when there is not json. (https://github.com/mangocubed/mango3/pull/203)

## [0.0.0-alpha.6] - 2025-02-07

### Added

- Show attached images at the bottom of the post. (https://github.com/mangocubed/mango3/pull/189)
- Store rendered content in a cache. (https://github.com/mangocubed/mango3/pull/191, https://github.com/mangocubed/mango3/pull/192, https://github.com/mangocubed/mango3/pull/193)
- Store blob queries in a cache. (https://github.com/mangocubed/mango3/pull/194)
- Store user queries in a cache. (https://github.com/mangocubed/mango3/pull/196)
- Store website queries in a cache. (https://github.com/mangocubed/mango3/pull/197)
- Store post queries in a cache. (https://github.com/mangocubed/mango3/pull/198)
- Store user session queries in a cache. (https://github.com/mangocubed/mango3/pull/199)

### Changed

- Bump lettre from 0.11.11 to 0.11.12. (https://github.com/mangocubed/mango3/pull/188)
- Bump uuid from 1.12.1 to 1.13.1. (https://github.com/mangocubed/mango3/pull/190)
- Set env var LEPTOS_WASM_OPT_VERSION=version_122 in GitHub Actions. (https://github.com/mangocubed/mango3/pull/200)

## [0.0.0-alpha.5] - 2025-02-04

### Added

- Dependencies badge and screenshot to README.md. (https://github.com/mangocubed/mango3/pull/185)

### Changed

- Bump openssl from 0.10.68 to 0.10.70 in the cargo group. (https://github.com/mangocubed/mango3/pull/181)
- Bump @types/node from 22.13.0 to 22.13.1. (https://github.com/mangocubed/mango3/pull/183)
- Bump clap from 4.5.27 to 4.5.28. (https://github.com/mangocubed/mango3/pull/184)
- Only delete user sessions when a user is locked. (https://github.com/mangocubed/mango3/pull/186)

### Fixed

- Apply correct orientation on a resized image. (https://github.com/mangocubed/mango3/pull/182)

## [0.0.0-alpha.4] - 2025-02-03

### Added

- Environment variable to choose filter for image resizing. (https://github.com/mangocubed/mango3/pull/173)
- Notify user by email when a new session has started with their account. (https://github.com/mangocubed/mango3/pull/178)
- Show message with support email address when asking for an invitation code. (https://github.com/mangocubed/mango3/pull/179)

### Changed

- Bump @playwright/test from 1.50.0 to 1.50.1. (https://github.com/mangocubed/mango3/pull/177)
- Bump async-trait from 0.1.85 to 0.1.86. (https://github.com/mangocubed/mango3/pull/174)
- Bump @types/node from 22.12.0 to 22.13.0 (https://github.com/mangocubed/mango3/pull/176)

## [0.0.0-alpha.3] - 2025-02-01

### Added

- UserTagLink component. (https://github.com/mangocubed/mango3/pull/168)
- CLI command to lock users and delete their information. (https://github.com/mangocubed/mango3/pull/169)

### Changed

- Bump leptos from 0.7.4 to 0.7.5. (https://github.com/mangocubed/mango3/pull/170)

### Fixed

- Make the website on PostCard more visible. (https://github.com/mangocubed/mango3/pull/171)

## [0.0.0-alpha.2] - 2025-01-30

### Added

- Optionally show post preview before submit. (https://github.com/mangocubed/mango3/pull/165)

### Changed

- Bump rand from 0.8.5 to 0.9.0. (https://github.com/mangocubed/mango3/pull/161)
- Improve README.md a little bit. (https://github.com/mangocubed/mango3/pull/162)
- Bump @types/node from 22.10.10 to 22.12.0. (https://github.com/mangocubed/mango3/pull/164)
- Bump serde_json from 1.0.137 to 1.0.138. (https://github.com/mangocubed/mango3/pull/163)

## [0.0.0-alpha.1] - 2015-01-27

### Added

- FUNDING.yml file. (https://github.com/mangocubed/mango3/pull/153)
- Controller for InfiniteScroll. (https://github.com/mangocubed/mango3/pull/157)

### Changed

- Replace lazy_static with LazyLock. (https://github.com/mangocubed/mango3/pull/155)
- Bump fred from 10.0.3 to 10.0.4. (https://github.com/mangocubed/mango3/pull/154)

## [0.0.0-alpha.0] - 2025-01-26

### Added

- Everything is new.
- Basic setup.
- Core package with database connection, models, configurations and other things. (https://github.com/mangocubed/mango3/pull/128, https://github.com/mangocubed/mango3/pull/148, https://github.com/mangocubed/mango3/pull/149, https://github.com/mangocubed/mango3/pull/150, https://github.com/mangocubed/mango3/pull/151)
- Leptos utility package with shared components, server side functions and other resources. (https://github.com/mangocubed/mango3/pull/54, https://github.com/mangocubed/mango3/pull/55, https://github.com/mangocubed/mango3/pull/58, https://github.com/mangocubed/mango3/pull/123, https://github.com/mangocubed/mango3/pull/132, https://github.com/mangocubed/mango3/pull/133, https://github.com/mangocubed/mango3/pull/146)
- Home application to browse websites, posts and users. (https://github.com/mangocubed/mango3/pull/64, https://github.com/mangocubed/mango3/pull/84, https://github.com/mangocubed/mango3/pull/87, https://github.com/mangocubed/mango3/pull/102, https://github.com/mangocubed/mango3/pull/145, https://github.com/mangocubed/mango3/pull/147)
- Monitor application to manage background jobs.
- Accounts application to register, login and reset password.
- Uploads application to display uploaded images.
- Studio application to manage websites, posts and navigation. (https://github.com/mangocubed/mango3/pull/12, https://github.com/mangocubed/mango3/pull/18, https://github.com/mangocubed/mango3/pull/19, https://github.com/mangocubed/mango3/pull/48, https://github.com/mangocubed/mango3/pull/88, https://github.com/mangocubed/mango3/pull/118, https://github.com/mangocubed/mango3/pull/134, https://github.com/mangocubed/mango3/pull/140)
- My Account application to manage user settings.
- Websites application to display websites and their posts. (https://github.com/mangocubed/mango3/pull/20, https://github.com/mangocubed/mango3/pull/21, https://github.com/mangocubed/mango3/pull/22, https://github.com/mangocubed/mango3/pull/30, https://github.com/mangocubed/mango3/pull/32, https://github.com/mangocubed/mango3/pull/60, https://github.com/mangocubed/mango3/pull/75, https://github.com/mangocubed/mango3/pull/85, https://github.com/mangocubed/mango3/pull/95, https://github.com/mangocubed/mango3/pull/100, https://github.com/mangocubed/mango3/pull/114, https://github.com/mangocubed/mango3/pull/139)
- CLI application to create invitation codes.
- Optionally, allow register only with invitation code.
- GitHub Actions workflows for continuous integration. (https://github.com/mangocubed/mango3/pull/1, https://github.com/mangocubed/mango3/pull/3, https://github.com/mangocubed/mango3/pull/6, https://github.com/mangocubed/mango3/pull/9, https://github.com/mangocubed/mango3/pull/27, https://github.com/mangocubed/mango3/pull/39, https://github.com/mangocubed/mango3/pull/59, https://github.com/mangocubed/mango3/pull/74)
- Image uploader. (https://github.com/mangocubed/mango3/pull/76)
- Websites themes. (https://github.com/mangocubed/mango3/pull/67, https://github.com/mangocubed/mango3/pull/68)
- Support for Handlebars in posts content. (https://github.com/mangocubed/mango3/pull/113)
- Hashtags on websites' description, posts content and users bio. (https://github.com/mangocubed/mango3/pull/109)
- Post views. (https://github.com/mangocubed/mango3/pull/61, https://github.com/mangocubed/mango3/pull/63)
- Post comments. (https://github.com/mangocubed/mango3/pull/137)
- Post reactions. (https://github.com/mangocubed/mango3/pull/142)
