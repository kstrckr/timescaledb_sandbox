FROM rust:1.50 AS rust-builder
WORKDIR /usr/src/mock_devices_rust_app
COPY ./mock_devices/mock_devices_rust_app .
RUN cargo run

FROM timescale/timescaledb:2.1.0-pg13
ADD ./timescaledb/003_init_mockdevices.sql /docker-entrypoint-initdb.d
ADD ./timescaledb/004_seed_data_from_csv.sql /docker-entrypoint-initdb.d

WORKDIR /usr
RUN mkdir seed_data
COPY --from=rust-builder /usr/src/mock_devices_rust_app/devices.csv /usr/seed_data
COPY --from=rust-builder /usr/src/mock_devices_rust_app/device_data.csv /usr/seed_data