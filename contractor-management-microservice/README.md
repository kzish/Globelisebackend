# Contractor Management Microservice

Microservice for the management of contractors

## Requirements

- [Rust compiler](https://www.rust-lang.org/tools/install)
- [Dapr](https://docs.dapr.io/getting-started/)
  - Depends on Docker
- Environment variables:
  - `LISTENING_ADDRESS`: IP address and port that the server will listen on
    - e.g. `localhost:3001`
    - All occurences of `localhost` will be replaced by `127.0.0.1`
  - `DATABASE_URL`: URL for connecting to the PostgreSQL database
    - e.g. `postgres://postgres:<password>@localhost/globelise_eor_admin_management`
  - `FRONTEND_URL`: URL of frontend
  - `USER_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the Dapr sidecar for user management microservice
  - `CONTRACTOR_MANAGEMENT_MICROSERVICE_DOMAIN_URL`: URL of the Dapr sidecar for contractor management microservice
  - `EOR_ADMIN_MICROSERVICE_DOMAIN_URL`: URL of the Dapr sidecar for EOR admin microservice
