use yaml_rust::{Yaml};
use super::utils::ts;
use urbit_http_api::{ShipInterface, Message, Messaging};

/// routing function for alerting planets, alert method
pub fn alerting_receiver(planets: &str, alert_type: &str, config_yaml: &Yaml) {
  println!("{} - Alert triggered - planets: {} type: {}", ts(), planets, alert_type);
  match alert_type {
    "text_alert" => text_alert(planets, config_yaml),
    "email_alert" => println!("email alert - coming soon?"),
    "urbit_alert" => urbit_alert(planets, config_yaml),
    &_ => println!("Unknown alert type"),
  }
}

/// send text message alert via textbelt API
fn text_alert(planets: &str, config_yaml: &Yaml){
  let alerting_endpoint = config_yaml["endpoint"].as_str().expect("Alerting endpoint not defined");
  let alerting_token = config_yaml["token"].as_str().expect("Alerting token not defined");
  let phone_number = config_yaml["phone_number"].as_i64().expect("Alerting phone_number not defined");
  let alert_text = config_yaml["alert_text"].as_str().expect("Alerting alert_text not defined");
  let message_pre: String = alert_text.to_string();
  let message_post = message_pre + planets;
  println!("{} - {} {} {}{}", ts(), alerting_endpoint, phone_number, alert_text, planets);
  

  let agent = ureq::Agent::new();
  let alert_status = agent.post(alerting_endpoint)
  .send_json(ureq::json!({
    "phone" : phone_number,
    "message" : message_post,
    "key" : alerting_token
  }));
  
  if alert_status.is_ok() {
    println!("{} - Alert sent [OK]", ts());
  }else{
    println!("{} - Alert failed to send [ERROR]", ts());
  }
}

/// post alert to reporter ship chat e.g. alerts-6170
fn urbit_alert(planets: &str, config_yaml: &Yaml){
  // set vars for reporter(target) ship
  let reporter_ship_address = config_yaml["reporter_ship_address"].as_str().expect("Reporter ship address not defined");
  let reporter_ship_code = config_yaml["reporter_ship_code"].as_str().expect("Reporter ship code not defined");
  let reporter_ship_name = config_yaml["reporter_ship_name"].as_str().expect("Reporter ship name not defined");
  let reporter_group_name = config_yaml["reporter_group_name"].as_str().expect("Reporter ship group name not defined");
  let reporter_alert_text = config_yaml["reporter_alert_text"].as_str().expect("Reporter alert text not defined");
  
  println!("{} - Sending alert to: {}, chat: {}", ts(), reporter_ship_name, reporter_group_name);
  
  // build connection to reporting ship
  let ship_interface = ShipInterface::new(reporter_ship_address, reporter_ship_code).unwrap();
  let mut channel = ship_interface.create_channel().unwrap();

  // build alert message, send alert
  let alert = Message::new()
  .add_text(reporter_alert_text)
  .add_text(planets);
  let _mess_res = channel
    .chat()
    .send_message(reporter_ship_name, reporter_group_name, &alert);
  channel.delete_channel();
}