# EOR Admin Microservice

Repository for EOR Admin Backend

## API

See [API](API.md).

## Requirements

- [Rust compiler](https://www.rust-lang.org/tools/install)
- [Dapr](https://docs.dapr.io/getting-started/)
  - Depends on Docker
- Ed25519 key pair for JWT encoding/decoding, stored in the files `private.pem` and `public.pem`
  - To generate them with OpenSSL, run these commands in the project root:
  ```
  openssl genpkey -algorithm ed25519 -outform PEM -out private.pem
  openssl pkey -in private.pem -outform PEM -pubout -out public.pem
  ```
- Environment variables:
  - `LISTENING_ADDRESS`: IP address and port that the server will listen on
    - e.g. `localhost:4002`
    - All occurences of `localhost` will be replaced by `127.0.0.1`
  - `DATABASE_URL`: URL for connecting to the PostgreSQL database
    - e.g. `postgres://postgres:<password>@localhost/globelise_eor_admin_management`
  - `GOOGLE_CLIENT_ID`: Google client ID
  - `GLOBELISE_SENDER_EMAIL`: Email address that will be used
  - `GLOBELISE_SMTP_USERNAME`: SMTP username
  - `GLOBELISE_SMTP_PASSWORD`: SMTP password
  - `GLOBELISE_SMTP_URL`: SMTP server URL
  - `FRONTEND_URL`: URL of frontend
    - e.g. `https://globelise.com`
  - `PASSWORD_RESET_URL`: URL of frontend password reset page
    - e.g. `https://globelise.com/resetpassword`
  - `USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the Dapr sidecar for user management microservice
  - `CONTRACTOR_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the Dapr sidecar for contractor management microservice
  - `EOR_ADMIN_MICROSERVICE_DOMAIN_URL`: URL of the Dapr sidecar for EOR admin microservice

## Build

```
cargo build
```

## Run

Inside the project's root directory, run the following command:

```
dapr run --app-id eor-admin-microservice --app-port 4002 --dapr-http-port 3992 --components-path ./components target/debug/eor-admin-microservice
```

If it is not starting, you may have to run Dapr using `sudo` for it to work properly.
