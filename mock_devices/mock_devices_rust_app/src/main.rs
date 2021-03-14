use std::time::{SystemTime, UNIX_EPOCH};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::error::Error;
use std::ffi::OsString;
use std::process;

mod mock_device;

use mock_device::MockDevice;

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}

// fn run() -> Result<(), Box<dyn Error>> {
//     let file_path = get_first_arg()?;
//     let mut wtr = csv::Writer::from_path(file_path)?;
//     wtr.write_record(&["id", "measurement", "timestamp"])?;
//     let mock_devices: Vec<MockDevice> = MockDevice::batch_of_mock_devices(10);

//     let start = SystemTime::now();
//     let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

//     for i in 0..8640 {
//         let interval_increment = 30000 * i;
//         for device in &mock_devices {
//             let timestamp = since_epoch.as_millis() - 2629800000 + interval_increment;
//             let measurement: (u64, u128) = device.get_next_data_point(timestamp);
//             let id_string: String = device.id.to_string(); 
//             let measurement_string: String = measurement.0.to_string();
//             let timestamp_string: String = measurement.1.to_string();

//             let id_str = &id_string[..];
//             let measurement_str = &measurement_string[..];
//             let timestamp_str = &timestamp_string[..];
//             wtr.write_record(&[id_str, measurement_str, timestamp_str])?;
//             // println!("{} {} {:?}", device.id, timestamp, measurement);
            
//         }
//     }
//     wtr.flush()?;
//     Ok(())
// }

// fn main() {
//     if let Err(err) = run() {
//         println!("{}", err);
//         process::exit(1);
//     }
// }

#[async_std::main]
// or #[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let mock_devices: Vec<MockDevice> = MockDevice::batch_of_mock_devices(100);

    // Create a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect("postgres://postgres:password@localhost").await?;

    // Make a simple query to return the given parameter
    // let row: Vec<Rate> = sqlx::query_as("SELECT * from rates")
    //     .fetch_all(&pool).await?;

    // assert_eq!(row.0, 150);
    // println!("{:?}", row);

    // for rate in &row {
    //     println!("{:?}\n", rate)
    // }
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

    // for device in mock_devices {
    //     let timestamp = since_epoch.as_millis() - 2629800000 + device.offset;
    //     let insert_string = format!("INSERT INTO mockdevices VALUES ('{}', '{}')", device.id, timestamp);
    //     // println!("{}", insert_string);
    //     let insert = sqlx::query(&insert_string)
    //         .execute(&pool).await?;
    // }

    for i in 0..8640 {
        let interval_increment = 30000 * i;
        for device in &mock_devices {
            let timestamp = since_epoch.as_millis() - 2629800000 + interval_increment;
            let measurement: (u64, u128) = device.get_next_data_point(timestamp);

            let insert_string = format!("INSERT INTO mockdevices VALUES ('{}', '{}', '{}')", device.id, measurement.1, measurement.0);
            // println!("{}", insert_string);
            let insert = sqlx::query(&insert_string)
                .execute(&pool).await?;
        }
    }

    Ok(())
}