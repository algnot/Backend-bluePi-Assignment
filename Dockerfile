FROM rust:1.83-bullseye AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
COPY src ./src

RUN cargo build --release
RUN cargo install diesel_cli

FROM debian:bullseye-slim
WORKDIR /app

RUN apt-get update && apt-get install -y libmariadb3 libssl-dev libc6

COPY --from=builder /app/target/release/backend-bluepi-assignment /app/backend-bluepi-assignment

EXPOSE 8080
CMD ["./wait-for-db.sh", "db:3306", "--", "./backend-bluepi-assignment"]
