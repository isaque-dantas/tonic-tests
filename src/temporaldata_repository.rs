use std::alloc::System;
use crate::models::TemporalDataFromDB;
use crate::temporal_data::{TemporalDataRequest};
use chrono::{FixedOffset};
use diesel::PgConnection;
use diesel::data_types::PgTimestamp;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::time::SystemTime;

const MICROSECONDS_IN_A_SECOND: i64 = 1_000_000;
const SECONDS_IN_A_MINUTE: i64 = 60;
const MINUTES_IN_AN_HOUR: i64 = 60;
const HOURS_IN_A_DAY: i64 = 24;
const DAYS_IN_A_YEAR: i64 = 365;
const YEARS_BETWEEN_1970_AND_2000: i64 = 30;

const DIFFERENCE_OF_STARTING_POINT_BETWEEN_POSTGRES_AND_CHRONO: i64 = DAYS_IN_A_YEAR
    * HOURS_IN_A_DAY
    * MINUTES_IN_AN_HOUR
    * SECONDS_IN_A_MINUTE
    * MICROSECONDS_IN_A_SECOND
    * YEARS_BETWEEN_1970_AND_2000;

pub fn save_data(data_request: TemporalDataRequest) -> Option<Vec<TemporalDataFromDB>> {
    let connection = &mut establish_connection();
    use crate::models::TemporalDataForm;
    use crate::schema::temporaldata::dsl::*;

    let data = Vec::from(data_request.data);
    let model_data: Vec<TemporalDataForm> = data
        .iter()
        .map(|d| TemporalDataForm {
            timestamp: get_time_from_string(&d.timestamp),
            value: d.value,
        })
        .collect();

    match diesel::insert_into(temporaldata)
        .values(&model_data)
        .get_results::<TemporalDataFromDB>(connection)
    {
        Ok(result) => Some(result),
        Err(err) => {
            dbg!(err);
            None
        }
    }
}

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn get_time_from_string(timestamp: &String) -> SystemTime {
    use chrono::DateTime;
    let datetime: DateTime<FixedOffset> = match DateTime::parse_from_str(timestamp.as_str(), "%+") {
        Ok(d) => d,
        Err(_) => panic!("Error parsing timestamp"),
    };

    datetime.into()
}
