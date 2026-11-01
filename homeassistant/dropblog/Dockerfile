# Home Assistant app image.
# Build context is the app folder, assembled by homeassistant/install.sh
# (app files plus Cargo.toml, Cargo.lock and src/).
FROM alpine:3.22 AS builder

RUN apk add --no-cache \
    rust \
    cargo \
    musl-dev \
    gcc

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM alpine:3.22 AS runtime

RUN apk add --no-cache ca-certificates imagemagick imagemagick-heic imagemagick-jpeg jq tzdata
COPY --from=builder /app/target/release/dropblog /usr/local/bin/dropblog
COPY run.sh /run.sh
RUN chmod a+x /run.sh

CMD ["/run.sh"]
