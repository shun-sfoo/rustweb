use std::time::SystemTime;

use chrono::NaiveDateTime;

fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64;

    println!("{}", now);

    convert();
}

fn convert() {
    let date = "2022-06-01".to_string();

    let begin = format!("{} 00:00:00", date);

    let res: NaiveDateTime =
        NaiveDateTime::parse_from_str(begin.as_str(), "%Y-%m-%d %H:%M:%S").unwrap();

    // 1654606673607007908
    // 1654606745350509789
    // 1654041600
    println!("{}", res.timestamp());
}
