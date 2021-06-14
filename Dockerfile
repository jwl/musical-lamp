# Compile stage
FROM rust:1.49 as builder

WORKDIR app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

# Runtime stage
FROM rust:1.49-slim as runtime
WORKDIR app
COPY --from=builder /app/target/release/musical_lamp musical_lamp
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./musical_lamp"]
