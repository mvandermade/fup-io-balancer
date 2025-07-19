# made-funicular-postzegel-io-router

## 🏗️ Got some ideas in docs/design

## Setup

* Must install
  * `protoc`
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
- grpc
- cli (clap)
- stats

