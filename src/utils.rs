use chrono::{Local};

/// generate timestamp
pub fn ts() -> std::string::String{
  return Local::now().format("%Y-%m-%dT%H:%M:%S").to_string();
}

/// append planet to String of alerting planets
pub fn add_planet_alert(alerting_planets: &mut String, new_planet: &str){
  alerting_planets.push_str(new_planet);
  alerting_planets.push_str(" ");
}