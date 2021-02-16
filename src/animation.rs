use chrono::Utc;

mod line;

pub fn start() {
  let oldest_timestamp: i64 = Utc::now().timestamp_millis();
  loop {
    let latest_timestamp: i64 = Utc::now().timestamp_millis();
    let running_timestamp: u64 = (latest_timestamp - oldest_timestamp) as u64;
    if running_timestamp > 1000 {
      break;
    }
    redraw(running_timestamp);
  }
}

fn redraw(timestamp: u64) {
  let line = line::Line {
    index: timestamp,
    content: String::from("          "),
  };
  println!("{} -> {}", line.get_index(), line.get_content());
}
