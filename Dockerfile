# syntax=docker/dockerfile:1

# Builds one provider: docker build --build-arg PROVIDER=cloudflare .
ARG RUST_VERSION=1.97

FROM rust:${RUST_VERSION}-alpine AS chef
RUN apk add --no-cache build-base cmake perl ca-certificates \
    && cargo install cargo-chef --locked
WORKDIR /src

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS build
ARG PROVIDER
RUN test -n "$PROVIDER" || (echo "--build-arg PROVIDER=<name> is required" && exit 1)
COPY --from=planner /src/recipe.json recipe.json
RUN cargo chef cook --locked --release -p "${PROVIDER}-provider" --recipe-path recipe.json
COPY . .
RUN cargo build --locked --release -p "${PROVIDER}-provider" --bin main \
    && cp target/release/main /provider \
    && ! readelf -d /provider | grep -q NEEDED

FROM scratch
COPY --from=build /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=build /provider /provider
EXPOSE 8080
USER 65532:65532
ENTRYPOINT ["/provider"]
