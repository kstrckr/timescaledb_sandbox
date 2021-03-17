extern crate chrono;
use chrono::prelude::*;
use uuid::Uuid;

// use sqlx::postgres::PgPoolOptions;
use std::time::{SystemTime, UNIX_EPOCH};
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


fn run() -> Result<(), Box<dyn Error>> {
    let seconds_in_a_month: i64 = 2629800;
    let fivemin_intervals_in_a_month: i64 = 8766;
    let data_file_name = OsString::from("device_data.csv");
    let devices_file_name = OsString::from("devices.csv");
    let mut data_wtr = csv::Writer::from_path(data_file_name)?;
    let mut device_wtr = csv::Writer::from_path(devices_file_name)?;
    let mock_devices: Vec<MockDevice> = MockDevice::batch_of_mock_devices(10000);

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let thirty_days_ago_unix_secs: i64 = since_epoch.as_secs() as i64 - seconds_in_a_month;


        for device in &mock_devices {
            let id_string: String = device.id.to_string();
            let id_str: &str = &id_string[..];
            let uid_string = device.uid.to_hyphenated().encode_lower(&mut Uuid::encode_buffer()).to_string();
            let uid_str = &uid_string[..];
            let type_string = device.device_type.to_string();
            let type_str = &type_string[..];
            let active_string = device.active.to_string();
            let active_str: &str = &active_string[..];
            device_wtr.write_record(&[id_str, uid_str, type_str, active_str])?;
            for i in 0..fivemin_intervals_in_a_month {
                let interval_increment = 300 * i;
                let five_min_increment = thirty_days_ago_unix_secs + interval_increment;
                let measurement: (u64, i64) = device.get_next_data_point(five_min_increment);
                let id_string: String = device.id.to_string(); 
                let measurement_string: String = measurement.0.to_string();

                // timestamp ms since epoch to datetime string here
                let naive_datetime = NaiveDateTime::from_timestamp(measurement.1, 0);
                let datetime: DateTime<Utc> = DateTime::from_utc(naive_datetime, Utc);
                let timestamp_string: String = datetime.to_string();

                let id_str = &id_string[..];
                let measurement_str = &measurement_string[..];
                let timestamp_str = &timestamp_string[..];
                data_wtr.write_record(&[id_str, timestamp_str, measurement_str])?;
            // println!("{} {} {:?}", device.id, timestamp, measurement);
            }
            data_wtr.flush()?;
        }
    device_wtr.flush()?;
    Ok(())
}

fn main() {
    let start = SystemTime::now();
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
    let finish = SystemTime::now();
    let difference = finish.duration_since(start)
        .expect("System clock may have self-updated during run resulting in negative duration");
    println!("{:?}", difference);
}

// #[async_std::main]
// // or #[tokio::main]
// async fn main() -> Result<(), sqlx::Error> {

//     let mock_devices: Vec<MockDevice> = MockDevice::batch_of_mock_devices(100);

//     // Create a connection pool
//     let pool = PgPoolOptions::new()
//         .max_connections(20)
//         .connect("postgres://postgres:password@localhost").await?;

//     // Make a simple query to return the given parameter
//     // let row: Vec<Rate> = sqlx::query_as("SELECT * from rates")
//     //     .fetch_all(&pool).await?;

//     // assert_eq!(row.0, 150);
//     // println!("{:?}", row);

//     // for rate in &row {
//     //     println!("{:?}\n", rate)
//     // }
//     let start = SystemTime::now();
//     let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time went backwards");

//     for i in 0..8640 {
//         let interval_increment = 30000 * i;
//         for device in &mock_devices {
//             let timestamp = since_epoch.as_millis() - 2629800000 + interval_increment;
//             let measurement: (u64, u128) = device.get_next_data_point(timestamp);

//             let insert_string = format!("INSERT INTO mockdevices VALUES ('{}', '{}', '{}')", device.id, measurement.1, measurement.0);
//             // println!("{}", insert_string);
//             let insert = sqlx::query(&insert_string)
//                 .execute(&pool).await?;
//         }
//     }

//     Ok(())
// }