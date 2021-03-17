use rand::{thread_rng, Rng};
use uuid::Uuid;

#[derive(Debug)]
pub struct MockDevice {
  pub id: u32,
  pub offset: i64,
  pub uid: Uuid,
  pub device_type: String,
  pub active: bool,
}

impl MockDevice {
  pub fn new(id: u32) -> Self {
    let new_uid = Uuid::new_v4();
    Self {
      id,
      offset: 0,
      uid: new_uid,
      device_type: "battery".to_string(),
      active: true,
    }
  }

  fn new_with_offset(id: u32, offset: i64) -> Self {
    Self {
      id,
      offset,
      uid: Uuid::new_v4(),
      device_type: "battery".to_string(),
      active: true,
    }
  }

  pub fn get_next_data_point(&self, base_time: i64) -> (u64, i64) {
    let mut rng = thread_rng();
    let new_timestamp = self.offset + base_time;
    let measurement: u64 = rng.gen_range(0, 500);
    (measurement, new_timestamp)
  }

  pub fn batch_of_mock_devices(count: u32) -> Vec<MockDevice> {
    let mut rng = thread_rng();
    let mut devices: Vec<MockDevice> = Vec::new();
    let count = count + 1;
    for i in 1..count {
      let n: i64 = rng.gen_range(0, 300);
      devices.push(MockDevice::new_with_offset(i, n));
    }
    devices
  }
}

