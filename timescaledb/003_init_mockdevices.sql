CREATE TABLE IF NOT EXISTS devices(
  id INTEGER PRIMARY KEY,
  uid UUID UNIQUE NOT NULL,
  type VARCHAR(20) NOT NULL,
  active BOOLEAN DEFAULT true
);

CREATE INDEX ON devices(id);

CREATE TABLE IF NOT EXISTS device_data(
  device_id INTEGER NOT NULL,
  time TIMESTAMP WITHOUT TIME ZONE NOT NULL,
  datapoint_value INTEGER NOT NULL,
  CONSTRAINT fk_id
    FOREIGN KEY(device_id)
      REFERENCES devices(id)
);
SELECT create_hypertable('device_data', 'time', chunk_time_interval => INTERVAL '1 day');
CREATE INDEX ON device_data (device_id, time desc);
CREATE INDEX ON device_data (time desc, device_id);