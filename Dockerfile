FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release

CMD ["./target/release/my_actix_project"]
