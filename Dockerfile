FROM alpine:3.8
WORKDIR /app

ADD target/release/server .

CMD ["/app/server" ]
