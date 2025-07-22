# made-funicular-postzegel-io-router

## 🏗️ Got some ideas in docs/design

## Setup

* Must install
  * `protoc` (`brew install protobuf`)
  * `cargo`/`rust`

* Run server
  
  ```shell
  cargo run -- -b 0.0.0.0:7331
  ```

* Run client

  ```shell
  cargo run --bin client -- -a http://127.0.0.1:7331
  ```

* If proto seems out of sync, do `cargo clean` and do 'cargo refresh' in RustRover if applicable.

# Todo
- make client disconnect gracefully
- drain backlog
- handle ack timeouts responses
- stats

# Docker
Use the Dockerfile to build an image for your architecture. Or run a GitHub action and publish it.