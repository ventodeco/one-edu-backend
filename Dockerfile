FROM scratch
COPY /target/x86_64-unknown-linux-musl/release/one-edu-backend /
CMD ["./one-edu-backend"]

# Expose the application port
EXPOSE 8000
