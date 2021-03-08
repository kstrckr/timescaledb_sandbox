use std::time::{SystemTime, UNIX_EPOCH};

use sqlx::postgres::PgPoolOptions;

mod mock_device;

use mock_device::MockDevice;

#[async_std::main]
// or #[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let mock_devices: Vec<MockDevice> = MockDevice::batch_of_mock_devices(100000);

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

    for device in mock_devices {
        let timestamp = since_epoch.as_millis() - 2629800000 + device.offset;
        let insert_string = format!("INSERT INTO mockdevices VALUES ('{}', '{}')", device.id, timestamp);
        // println!("{}", insert_string);
        let insert = sqlx::query(&insert_string)
            .execute(&pool).await?;
    }

    Ok(())
}