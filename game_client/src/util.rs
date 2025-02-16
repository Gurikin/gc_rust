use std::time::SystemTime;

use chrono::{DateTime, Utc};

pub fn get_format_time(format: Option<&str>) -> String {
    let now = SystemTime::now();
    let datetime: DateTime<Utc> = now.into();
    format!("{}", datetime.format(format.unwrap_or("%d/%m/%Y'T'%T")))
}
