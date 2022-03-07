# User Management Microservice

This microservice handles user authentication and authorization.

## API

See [API](API.md).

## Requirements

- [Rust compiler](https://www.rust-lang.org/tools/install)
- [Dapr](https://docs.dapr.io/getting-started/)
  - Depends on Docker
- Environment variables:
  - `LISTENING_ADDRESS`: IP address and port that the server will listen on
    - e.g. `localhost:3000`
    - All occurences of `localhost` will be replaced by `127.0.0.1`
  - `DATABASE_URL`: URL for connecting to the PostgreSQL database
    - e.g. `postgres://postgres:<password>@localhost/globelise_user_management`
  - `GOOGLE_CLIENT_ID`: Google client ID
  - `GLOBELISE_DOMAIN_URL`: URL for the server hosting this microservice
  - `GLOBELISE_SMTP_EMAIL`: Email address for sending change password email
  - `GLOBELISE_SMTP_USERNAME`: SMTP username
  - `GLOBELISE_SMTP_PASSWORD`: SMTP password
  - `GLOBELISE_SMTP_URL`: SMTP server URL
  - `PASSWORD_RESET_URL`: URL of frontend password reset page

## Build

```
cargo build
```

## Run

Inside the project's root directory, run the following command:

```
dapr run --app-id user-management-microservice --app-port 3000 --dapr-http-port 3500 --components-path ./components target/debug/user-management-microservice
```

If it is not starting, you may have to run Dapr using `sudo` for it to work properly.
