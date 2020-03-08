# -----------------
# Cargo Build Stage
# -----------------
FROM rust:1.41 as cargo-build
WORKDIR /app
COPY . .
RUN cargo build --release
# -----------------
# Final Stage
# -----------------
FROM debian:stable-slim
COPY --from=cargo-build /app/target/release/rustmemtest /bin
CMD ["rustmemtest"]