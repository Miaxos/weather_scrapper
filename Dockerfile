# -----------------
# Cargo Build Stage
# -----------------

FROM rg.fr-par.scw.cloud/brevz/rust-builder:1.56.1 as cargo-build

WORKDIR /usr/src/app
COPY Cargo.lock .
COPY Cargo.toml .

# RUN mkdir .cargo
# RUN cargo vendor > .cargo/config

COPY ./src src
RUN cargo build --release
RUN cargo install --path . --verbose

# -----------------
# Final Stage
# -----------------

# Copy the binary into a new container for a smaller docker image
FROM rg.fr-par.scw.cloud/brevz/debian-movine:2.0.0

RUN apt-get update
RUN apt-get install -y openssl ca-certificates

COPY ./auth.json auth.json
COPY --from=cargo-build /usr/local/cargo/bin/scrapper /bin

ENTRYPOINT ["scrapper"]
