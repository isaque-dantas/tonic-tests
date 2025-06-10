use std::time::SystemTime;
use diesel::{Insertable, Queryable};
use crate::schema::temporaldata;
use crate::temporal_data::TemporalDataMessage;

#[derive(Queryable)]
#[diesel(table_name = temporaldata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TemporalDataFromDB {
    pub id: i32,
    pub timestamp: SystemTime,
    pub value: f32
}

impl TemporalDataFromDB {
    pub fn to_message(&self) -> TemporalDataMessage {
        use chrono::DateTime;
        
        let time_elapsed = match self.timestamp.elapsed() {
            Ok(elapsed) => elapsed,
            Err(_) => panic!("Failed to get elapsed time")
        };
        
        let timestamp = DateTime::from_timestamp_micros(time_elapsed.as_micros() as i64);
        let timestamp = match timestamp {
            Some(t) => t,
            None => panic!("Failed to get timestamp")
        };
        
        TemporalDataMessage {
            value: self.value,
            timestamp: timestamp.to_string()
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = temporaldata)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TemporalDataForm {
    pub timestamp: SystemTime,
    pub value: f32
}
