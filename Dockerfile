FROM rust:1.64.0-bullseye as cargo
WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim as rt
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates
COPY --from=cargo /usr/local/cargo/bin/dropblog /usr/local/bin/
ENV TZ="Europe/London"
RUN apt-get update && apt-get install -y imagemagick
CMD ["dropblog"]
