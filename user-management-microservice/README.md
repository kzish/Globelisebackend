# User Management Microservice

This microservice handles user authentication and authorization.

## API

Work in progress...

## Requirements

- Dapr (see their [Getting started](https://docs.dapr.io/getting-started/) guide)
    - Note: For this project, session data should be on redis, and other data should be
    backed by PostgreSQL. Currently, this project is not configured correctly to use multiple
    stores, so as a workaround, data just gets multiplexed on a single redis store. **This must
    be fixed for production.**
- RSA key pair, stored in the files `private.pem` and `public.pem`
    - To generate them with OpenSSL, run these commands:
    ```
    openssl genrsa -out private.pem 2048
    openssl rsa -in private.pem -outform PEM -pubout -out public.pem
    ```
- Google client ID, stored in the `GOOGLE_CLIENT_ID` environment variable

## Build

```
cargo build
```

## Run

Inside the project's root directory, run the following command:

```
dapr run --app-id user-management-microservice --app-port 3000 --dapr-http-port 3500 --components-path ./components target/debug/user-management-microservice
```

The server will listen on `localhost:3000`.

If it is not starting, you may have to run Dapr using `sudo` for it to work properly.
