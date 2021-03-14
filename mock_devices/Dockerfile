FROM rust:1.50

WORKDIR /usr/src/mock_devices_rust_app
COPY ./mock_devices_rust_app .

RUN cargo install --path .
WORKDIR /usr
RUN mkdir temp
WORKDIR /usr/temp

CMD ["mock_devices_rust_app", "test.csv"]