# docker exec -it timescaledb psql -U postgres
FROM timescale/timescaledb:2.1.0-pg13
ADD 003_init_mockdevices.sql /docker-entrypoint-initdb.d
ADD 004_seed_data_from_csv.sql /docker-entrypoint-initdb.d