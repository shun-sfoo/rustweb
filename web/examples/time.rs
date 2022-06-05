use std::time::SystemTime;

fn main() {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos() as i64;

    println!("{}", now);
}
