## Install

These instructions will get you a project up and running on your local machine for development and testing purposes.

### Prerequisites

Be sure to have the following properly installed:

- [Docker](https://www.docker.com/)
- [Rust](https://www.rust-lang.org/tools/install) v1.70 or higher
- [cargo-make](https://github.com/sagiegurari/cargo-make) v0.36.3 or higher

### Setup

Build the client and backend services:

```sh
cargo make build-backend && \
cargo make build-client
```

### Launch

#### Client

To launch the client use the following command:

```sh
cargo make serve-client
```

#### Backend

To launch the backend use the following command:

```sh
cargo make serve-backend
```

Make sure you have your .env file ready with the necessairy variables. See .env.example in crate

#### Database

To launch the database see the [Database Readme](./database/README.md)

Make sure you have your .env file ready with the necessairy variables. See .env.example in folder
