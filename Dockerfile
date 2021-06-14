# Compile stage
FROM rust:1.49 AS builder

WORKDIR app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

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
