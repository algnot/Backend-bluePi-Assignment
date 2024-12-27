FROM rust:1.83-bullseye AS builder
WORKDIR /app

# Copy the Rust project files
COPY Cargo.toml Cargo.lock ./
RUN cargo fetch --locked
COPY . .

# Ensure `wait-for-db.sh` has executable permissions
RUN chmod +x ./wait-for-db.sh

# Install required libraries and Diesel CLI
RUN apt-get update && apt-get install -y \
    libmariadb3 libssl-dev libc6 \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install diesel_cli

# Expose the application port
EXPOSE 8080

# Command to start the container
CMD ["./wait-for-db.sh", "db:3306", "--", "cargo", "run"]
