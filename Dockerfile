# Development and CI image — not documented in README (host setup is the reference path).
FROM rust:1-bookworm

WORKDIR /app

COPY Cargo.toml ./
COPY src/mail.rs src/mail.rs
RUN echo "fn main() {}" > src/main.rs \
    && cargo build --release \
    && rm src/main.rs target/release/deps/actix_mailexam*

COPY . .
RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/actix-mailexam"]
