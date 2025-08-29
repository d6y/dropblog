FROM alpine:3.22 AS builder

RUN apk add --no-cache \
    rust \
    cargo \
    musl-dev \
    gcc

WORKDIR /app
COPY . .
RUN cargo build --release

FROM alpine:3.22 AS runtime

RUN apk add --no-cache ca-certificates imagemagick ffmpeg
COPY --from=builder /app/target/release/dropblog /usr/local/bin/dropblog

RUN addgroup -g 1000 appuser && adduser -D -s /bin/sh -u 1000 -G appuser appuser
RUN chown appuser:appuser /usr/local/bin/dropblog

USER appuser
ENV TZ="Europe/London"
CMD ["dropblog"]
