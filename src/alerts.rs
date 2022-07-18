
use std::io::{stdout, Write};
use curl::easy::Easy;
use linked_hash_map::LinkedHashMap;
use yaml_rust::{YamlLoader, YamlEmitter, Yaml};




//pub fn text_alert(config_yaml:&LinkedHashMap<Yaml,Yaml>){
pub fn text_alert(config_yaml: &str){
  let cfgs = YamlLoader::load_from_str(&config_yaml).unwrap();
  let cfg = &cfgs[0];
  //file.read_to_string(&mut monitor_configs).expect("Unable to read the file");
  //let cfgs = YamlLoader::load_from_str(&monitor_configs).unwrap();

  let alerting_endpoint = cfg["text-alerting"]["endpoint"].as_str().expect("Alerting endpoint not defined");
  let alerting_token = cfg["text-alerting"]["token"].as_str().expect("Alerting token not defined");
  let phone_number = cfg["text-alerting"]["phone_number"].as_str().expect("Alerting phone_number not defined");
  let alert_text = cfg["text-alerting"]["alert_text"].as_str().expect("Alerting alert_text not defined");

  println!("{} - {} {} {} {}", ts(), alerting_endpoint, alerting_token, phone_number, alert_text);

}