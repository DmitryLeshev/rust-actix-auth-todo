FROM debian:bullseye-slim
WORKDIR /app
ADD target/release/server .
CMD ["/app/server" ]

# ADD ../db .
# CMD ["/app/server", "sh db/setup.sh"]