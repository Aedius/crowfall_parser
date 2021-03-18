
FROM clux/muslrust:stable as builder
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser

# Build the project with target x86_64-unknown-linux-musl

RUN set -x && cargo install wasm-pack

# Build dummy main with the project's Cargo lock and toml
# This is a docker trick in order to avoid downloading and building
# dependencies when lock and toml not is modified.
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src \
    && echo "fn toto() {print!(\"Dummy main\");} // dummy file" > src/lib.rs

COPY src ./src

RUN wasm-pack build --target web --out-dir /pkg

RUN ls -l /pkg

FROM nginx:1.17.4-alpine

RUN rm /etc/nginx/conf.d/default.conf

COPY config/static.conf /etc/nginx/conf.d/default.conf
COPY static /usr/share/nginx/html/
COPY --from=builder /pkg /usr/share/nginx/html/pkg/


RUN ls -l /usr/share/nginx/html/
RUN ls -l /usr/share/nginx/html/pkg/

EXPOSE 80