FROM rust:1.49

WORKDIR app
COPY . .
RUN cargo build --release
ENTRYPOINT ["./target/release/musical_lamp"]
