FROM rust:alpine3.23 as builder

WORKDIR /usr/src/frp-operator
COPY . .

# build frp-operator binary
RUN cargo build --release -p frp-operator --bin frp-operator

FROM alpine:3.23
COPY --from=builder /usr/src/frp-operator/target/release/frp-operator /usr/local/bin/frp-operator

CMD ["frp-operator"]