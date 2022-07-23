use yaml_rust::{Yaml};
use super::utils::ts;

// receiver function to route alerts (when additional alerting added)
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