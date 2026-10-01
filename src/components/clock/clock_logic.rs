use chrono::{DateTime, Utc};
use chrono_tz::America::Sao_Paulo;

pub fn get_realtime() -> String {
    let time: DateTime<Utc> = Utc::now();
    let tz_time = time.with_timezone(&Sao_Paulo);
    let string_time = tz_time.format("%H:%M").to_string();
    string_time
}
