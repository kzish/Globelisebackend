# Contractor Management Microservice

Microservice for the management of contractors

## Requirements

- [Rust compiler](https://www.rust-lang.org/tools/install)
- [Dapr](https://docs.dapr.io/getting-started/)
  - Depends on Docker
- Environment variables:
  - `LISTENING_ADDRESS`: IP address and port that the server will listen on
    - e.g. `localhost:4001`
    - All occurences of `localhost` will be replaced by `127.0.0.1`
  - `DAPR_ADDRESS`: IP address and port that the DAPR sidecar is listening to
    - e.g. `localhost:3502`
  - `DATABASE_URL`: URL for connecting to the PostgreSQL database
    - e.g. `postgres://postgres:<password>@localhost/globelise_eor_admin_management`
  - `FRONTEND_URL`: URL of frontend
    - e.g. `https://globelise.com`
  - `USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the user microservice
  - `CONTRACTOR_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the contractor microservice
  - `EOR_ADMIN_MICROSERVICE_DOMAIN_URL`: URL of the admin microservice

## Build

```
cargo build
```

## Run

Inside the project's root directory, run the following command:

```
dapr run --app-id eor-admin-microservice --app-port 4001 --dapr-http-port 3991 --components-path ./components target/debug/eor-admin-microservice
```

If it is not starting, you may have to run Dapr using `sudo` for it to work properly.
