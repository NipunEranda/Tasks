FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo install --path .


FROM debian:buster-slim AS runner
COPY --from=builder /usr/local/cargo/bin/tasks /usr/local/bin/rust-rocket-tasks
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["tasks"]