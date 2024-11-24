FROM rust:1.82.0-bullseye as build

WORKDIR /build

COPY . .

RUN cargo build --release

FROM rust:1.82.0-bullseye as runtime

WORKDIR /app

COPY --from=build /build/target/release/paidy-submission .

CMD ["./paidy-submission"]