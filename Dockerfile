FROM rust:1.88-alpine3.20 AS builder
COPY . /opt/repo
WORKDIR /opt/repo
# To prevent crosstalk with local builds
RUN rm -rf target
RUN apk add musl-dev protoc
RUN cargo build --release

FROM scratch
COPY --from=builder /opt/repo/target/release/server /opt/app/server
CMD ["opt/app/server", "-v"]

