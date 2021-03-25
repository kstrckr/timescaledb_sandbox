  // const DIMENSION: u8 = 16385;
  use std::{thread, time};
  use rand::prelude::*;

const TWO: u32 = 2;

struct OutputMetadata {
  last_index: usize,
  n: u32,
  length: u32,
  max_value: u32,
}

impl OutputMetadata {
  pub fn new(length: u32, max_value: u32) -> OutputMetadata {
    let n = validate_length(length as f32).expect("Could not create valid output");

    OutputMetadata {
      last_index: length as usize - 1,
      n,
      length,
      max_value,
    }
  }
}

pub fn create_ds() -> Vec<u32> {
  generate_ds_vec(16385, 80)
}

fn validate_length(length: f32) -> Result<u32, String> {
  let mut valid: bool = false;
  let mut n: u32 = 0;
  
  if length >= 5.0 {
    let mut i = length - 1.0;
    while i > 1.0 {
      i = i / 2.0;
      n = n + 1;
      if i == 1.0 {
        valid = true;
        break;
      }
    }
  }
  if valid {
    return Ok(n);
  }
  Err(format!("{} does not conform to 2^N+1", length))
}

fn determine_offset(left: &u32, right: &u32, scale: u32, iteration: u32) -> u32 {
  let mut rng = thread_rng();
  let starting_value = (left + right) / 2;
  let attempt_minus = rng.gen_bool(0.5);
  let offset = rng.gen_range(0, scale / 4);
  match attempt_minus {
    true => {
      let minus_possible: bool = starting_value > offset;
      if minus_possible {
        return starting_value - offset;
      }
      return starting_value + offset;
    },
    false => return starting_value + offset,
  }
}

pub fn generate_ds_vec(output_length: u32, scale: u32) -> Vec<u32> {
  // TODO: randomness baseline
  let mut rng = thread_rng();

  // ERROR CHECKING
  let output_metadata = OutputMetadata::new(output_length, scale);

  // // initial setup
  
  let mut output: Vec<u32> = vec![0; output_length as usize];



  // let mut array: [u32; DIMENSION] = [0; DIMENSION];
  output[0] = rng.gen_range(0, scale);
  output[output_metadata.last_index] = rng.gen_range(0, scale);


  // // generate the data
  for i in 1..output_metadata.n + 1 {
    let pow = TWO.pow(i);
    for j in 1..pow {
      if j % 2 > 0 {
        let index_offset = (output_metadata.length - 1) / pow;
        let target_index = index_offset * j;
        let lower_source = target_index - index_offset;
        let upper_source = target_index + index_offset;

        // output[target_index as usize] = ((&output[lower_source as usize] + &output[upper_source as usize]) / 2) + rng.gen_range(0, (scale - i) / i);
        output[target_index as usize] = determine_offset(&output[lower_source as usize], &output[upper_source as usize], scale, 1);
      }
    }
  }
  // array
  output
}

pub fn print_midpoint_displaced_vec (elements: Vec<u32>) {
  let ten_millis = time::Duration::from_millis(10);

  for i in elements {
    let mut printed_row: String = String::new();
    for j in 0..i {
      printed_row.push('*');
    }
    println!("{}", printed_row);
    thread::sleep(ten_millis);
  }
}

#[cfg(test)]
mod tests {
  use super::*;
    #[test]
    fn validate_5_as_true() {
        assert_eq!(validate_length(5.0).is_ok(), true);
    }

    #[test]
    fn validate_6_as_false() {
        assert_eq!(validate_length(6.0).is_ok(), false);
    }

    #[test]
    fn validate_16385_as_true() {
        assert_eq!(validate_length(16385.0).is_ok(), true);
    }

    #[test]
    fn validate_123847032_as_false() {
        assert_eq!(validate_length(123847032.0).is_ok(), false);
    }

    #[test]
    #[should_panic]
    fn generate_ds_generic_invalid_length() {
      generate_ds_vec(13, 100);
    }

    #[test]
    fn output_length_is_correct() {
      assert_eq!(generate_ds_vec(17, 100).len(), 17)
    }

    #[test]
    fn initializes_first() {
      assert!(generate_ds_vec(17, 100)[0] > 0)
    }

    #[test]
    fn initializes_last() {
      assert!(generate_ds_vec(17, 100)[16] > 0)
    }

}
