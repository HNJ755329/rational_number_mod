FROM rust:1.80

WORKDIR /usr/src/myapp
# COPY . .

# RUN cargo install --path .
RUN cargo install wasm-pack

# CMD ["myapp"]
