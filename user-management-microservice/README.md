# User Management Microservice

This microservice handles user authentication and authorization.

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
    - e.g. `localhost:4000`
    - All occurences of `localhost` will be replaced by `127.0.0.1`
  - `DAPR_ADDRESS`: IP address and port that the DAPR sidecar is listening to
    - e.g. `localhost:3500`
  - `DATABASE_URL`: URL for connecting to the PostgreSQL database
    - e.g. `postgres://postgres:<password>@localhost/globelise_user_management`
  - `GOOGLE_CLIENT_ID`: Google client ID
  - `GLOBELISE_SENDER_EMAIL`: Email address that will be used
  - `GLOBELISE_SMTP_USERNAME`: SMTP username
  - `GLOBELISE_SMTP_PASSWORD`: SMTP password
  - `GLOBELISE_SMTP_URL`: SMTP server URL
  - `FRONTEND_URL`: URL of frontend
    - e.g. `https://globelise.com`
  - `USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the user microservice
  - `CONTRACTOR_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the contractor microservice
  - `EOR_ADMIN_MICROSERVICE_DOMAIN_URL`: URL of the admin microservice
<<<<<<< HEAD
  - `MULESOFT_API_URL`: URL to Mulesoft integration website
  - `MULESOFT_CLIENT_ID`: Mulesoft client ID
  - `MULESOFT_CLIENT_SECRET`: Mulesoft client secret
=======
>>>>>>> 8b6204abb3b6f44ed7084b36215c7e14eabae567

## Build

Inside this microservice's root directory, run the following command:

```
cargo build
```

## Run

```
dapr run --app-id user-management-microservice --app-port 4000 --dapr-http-port 3990 --components-path ./components ../target/debug/user-management-microservice
```

If it is not starting, you may have to run Dapr using `sudo` for it to work properly.
