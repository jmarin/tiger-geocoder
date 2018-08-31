# Build environment
FROM ekidd/rust-musl-builder as builder

# Fix permissions on source code
RUN sudo chown -R rust:rust /home/rust

# Trick to cache build dependencies
COPY Cargo.toml .
RUN mkdir -p src && echo > src/main.rs "fn main() {}"
RUN cargo build --release

# Copy source code
COPY src/ src/

# Build application
RUN cargo build --release

# Create Docker image, copying output from builder image
FROM scratch
#RUN apk --no-cache add ca-certificates
COPY --from=builder \
  /home/rust/src/target/x86_64-unknown-linux-musl/release/geocode \
  /
ENTRYPOINT ["/geocode"]

