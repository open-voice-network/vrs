# Voice Registry System (VRS)

This repository contains the VRS source code, swagger.

# Overview and Problem Context

VRS documentation, please see [here](https://github.com/open-voice-network/docs/blob/main/components/voice_registry_system.md)

# Prerequisites

## Required

- Download [Rust](https://www.rust-lang.org/tools/install)
- Framework: Rockets
- ORM: Diesel
- Log: Log, Log4rs
- Data store: MongoDB

## Optional

- An API Explorer ([Postman](https://www.postman.com/downloads) or [Insomnia](https://insomnia.rest/download))
- Install [cargo watch](https://crates.io/crates/cargo-watch). Cargo Watch watches over your project's source for changes, and runs Cargo commands when they occur. It is like a nodemon
- Install [mongodb] (https://docs.mongodb.com/manual/tutorial/install-mongodb-on-os-x/).
- Optional. Install [Robo 3T](https://robomongo.org/).

```sh
cargo install cargo-watch
```
## Log
- Log configuration file is log4rs.yml. A good [log4rs understanding](https://www.programmersought.com/article/83816316972/) and [log](https://docs.rs/log/0.4.6/log/).
- Please follow this [logging guidelines](https://betterprogramming.pub/production-grade-logging-in-rust-applications-2c7fffd108a6)

## Build/Run

1. Add .env file in the root folder
2. Set-up your database settings.

```
MONGODB_ADDRESS=[address]
MONGODB_PORT=[portnumber]
MONGODB_DATABASE=[yourdatabase]
MONGODB_USER=[username]
MONGODB_PASSWORD=[password]
```
3. Run the application
```sh
cargo run
```

OR

```sh
cargo watch -x run
```

## Docker build and run

```sh
docker build --progress=plain --tag open-voice-network/vrs:0.0.1 .
docker run -p 8000:8000 open-voice-network/vrs:0.0.1
```

Once the VRS container is running, check out the following URLs:

- <localhost:8000>
- <localhost:8000/health>
- <localhost:8000/api/records/>
- <localhost:8000/api/users/>
