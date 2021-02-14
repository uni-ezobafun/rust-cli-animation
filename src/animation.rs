use chrono::Utc;

pub fn start() {
  let oldest_timestamp: i64 = Utc::now().timestamp_millis();
  loop {
    let latest_timestamp: i64 = Utc::now().timestamp_millis();
    let running_timestamp: u64 = (latest_timestamp - oldest_timestamp) as u64;
    redraw(running_timestamp);
  }
}

fn redraw(t: u64) {
  println!("{}", t);
}
