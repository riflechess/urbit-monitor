use std::io::{stdout, Write, Read};
//use curl::easy::Easy;
use linked_hash_map::LinkedHashMap;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};
use super::utils::ts;
use ureq::{post, Request};

// receiver function to route alerts
pub fn alerting_receiver(planets: &str, alert_type: &str, config_yaml: &Yaml) {
  println!("{} - Alert triggered - planets: {} type: {}", ts(), planets, alert_type);
  match alert_type {
    "text_alert" => text_alert(planets, config_yaml),
    "email_alert" => println!("email alert!"),
    &_ => println!("Unknown alert type"),
  }
}

// send text message alert
fn text_alert(planets: &str, config_yaml: &Yaml){
  let alerting_endpoint = config_yaml["endpoint"].as_str().expect("Alerting endpoint not defined");
  let alerting_token = config_yaml["token"].as_str().expect("Alerting token not defined");
  let phone_number = config_yaml["phone_number"].as_i64().expect("Alerting phone_number not defined");
  let alert_text = config_yaml["alert_text"].as_str().expect("Alerting alert_text not defined");
  println!("{} - {} {} {}{}", ts(), alerting_endpoint, phone_number, alert_text, planets);

  let agent = ureq::Agent::new();
  let alert_status = agent.post(alerting_endpoint)
  .send_json(ureq::json!({
    "phone" : phone_number,
    "message" : alert_text,
    "key" : alerting_token
  }));
  
  if alert_status.is_ok() {
    println!("{} - Alert sent [OK]", ts());
  }else{
    println!("{} - Alert failed to send [ERR]", ts());
  }



    
}