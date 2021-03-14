use rand::{thread_rng, Rng};

#[derive(Debug)]
pub struct MockDevice {
  pub id: u32,
  pub offset: u128,
}

impl MockDevice {
  pub fn new(id: u32) -> Self {
    Self {
      id,
      offset: 0,
    }
  }

  fn new_with_offset(id: u32, offset: u128) -> Self {
    Self {
      id,
      offset,
    }
  }

  pub fn get_next_data_point(&self, base_time: u128) -> (u64, u128) {
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
      let n: u128 = rng.gen_range(0, 300000);
      devices.push(MockDevice::new_with_offset(i, n));
    }
    devices
  }
}

