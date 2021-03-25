extern crate diamond_square_lib;

use diamond_square_lib::diamond_square;
use rand::{thread_rng, Rng};
use uuid::Uuid;

pub struct DataPoint {
  pub value: u64,
  pub timestamp: i64,
}

#[derive(Debug)]
pub struct MockDevice {
  pub id: u32,
  pub offset: i64,
  pub uid: Uuid,
  pub device_type: String,
  pub active: bool,
  pub mock_data: Vec<u32>,
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
      mock_data: diamond_square::create_ds(),
    }
  }

  fn new_with_offset(id: u32, offset: i64) -> Self {
    Self {
      id,
      offset,
      uid: Uuid::new_v4(),
      device_type: "battery".to_string(),
      active: true,
      mock_data: diamond_square::create_ds(),
    }
  }

  pub fn get_next_data_point(&self, base_time: i64, index: i64) -> DataPoint {
    let timestamp = self.offset + base_time;
    let value = self.mock_data[index as usize] as u64;

    DataPoint { value, timestamp }
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

