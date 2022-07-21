// timestamp for logging
use chrono::{Local, DateTime, TimeZone};

pub fn ts() -> std::string::String{
  return Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
}