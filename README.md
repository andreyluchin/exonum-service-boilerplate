# Exonum Service Boilerplate

This repo serves as a boilerplate to be used with [`cargo-generate`](https://github.com/ashleygwilliams/cargo-generate)
entry point to using [Exonum](https://github.com/exonum/exonum)
\- an extensible open-source framework for creating private/permissioned blockchain applications (https://exonum.com).

## Usage

1. `cargo install cargo-generate`
2. `cargo generate --git https://github.com/exonum/exonum-service-boilerplate --name exonum-service`

## Project layout

* **blockchain** - connects blockchain related stuff together.
    - **models.rs** - where data layout using `encoding_struct`s is described.
    - **transactions.rs** - where transactions are described.
    - **schema.rs** - where the data schema is declared.
    - **errors.rs** - a place for blockchain-related errors.
* **api.rs** - REST API with Exonum methods.
* **service.rs** - Service Trait implementation along with some helpers.
* **main.rs** - entry point to the Exonum Service binary.
