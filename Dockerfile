# Plan stage
FROM lukemathwalker/cargo-chef as planner
WORKDIR app
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# Cacher stage
FROM lukemathwalker/cargo-chef as cacher
WORKDIR app
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies; NOT the application
RUN cargo chef cook --release --recipe-path recipe.json

# Compile stage
FROM rust:1.49 AS builder
WORKDIR app
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin musical_lamp

# Runtime stage
FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
          && apt-get install -y --no-install-recommends openssl \
          && apt-get autoremove -y \
          && apt-get clean -y \
          && rm -rf /var/lib/apt-lists/*
COPY --from=builder /app/target/release/musical_lamp musical_lamp
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./musical_lamp"]
