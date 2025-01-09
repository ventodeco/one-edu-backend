FROM messense/rust-musl-cross:x86_64-musl as builder
WORKDIR /oneedubackend
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /oneedubackend/target/x86_64-unknown-linux-musl/release/one-edu-backend /one-edu-backend
ENTRYPOINT [ "/one-edu-backend" ]
EXPOSE 8000
