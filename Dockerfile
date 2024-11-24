FROM rust:1.82.0-bullseye AS build

WORKDIR /build

COPY . .

RUN cargo build --release

FROM rust:1.82.0-bullseye AS runtime

WORKDIR /app

COPY --from=build /build/target/release/paidy-submission .

CMD ["./paidy-submission"]