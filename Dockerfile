FROM rust:latest

COPY . .

RUN cargo build 

CMD ["./target/debug/hello-world"]