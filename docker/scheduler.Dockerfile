FROM rust:1.78 as builder

WORKDIR /app
COPY . .

RUN cargo build -p gridflow-scheduler --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/gridflow /usr/local/bin/gridflow

CMD ["gridflow"]
