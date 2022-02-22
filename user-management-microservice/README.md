# User Management Microservice
This microservice handles user authentication and authorization.

## API
See [API](API.md).

## Requirements
- [Rust compiler](https://www.rust-lang.org/tools/install)
- [Dapr](https://docs.dapr.io/getting-started/)
    - Depends on Docker
- RSA key pair for JWT encoding/decoding, stored in the files `private.pem` and `public.pem`
    - To generate them with OpenSSL, run these commands in the project root:
    ```
    openssl genrsa -out private.pem 2048
    openssl rsa -in private.pem -outform PEM -pubout -out public.pem
    ```
- Environment variables:
    - `LISTENING_ADDRESS`: the IP address and port that the server will listen on
        - e.g. `localhost:3000`
        - All occurences of `localhost` will be replaced by `127.0.0.1`
    - `DATABASE_URL`: the URL for connecting to the PostgreSQL database
        - e.g. `postgres://postgres:<password>@localhost/globelise_user_management`
    - `GOOGLE_CLIENT_ID`: Google client ID
    - `GLOBELISE_SMTP_USERNAME`: SMTP username for sending password reset email
    - `GLOBELISE_SMTP_PASSWORD`: SMTP password for sending password reset email

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
