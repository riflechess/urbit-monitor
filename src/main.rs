#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
//use urbit-http-api::{};

extern crate exitcode;
extern crate system_config;
//extern crate serde_yaml;
extern crate yaml_rust;

//use std::fs::File;
//use std::io::Read;
use std::env;
use std::path::Path;
use serde::{Deserialize, Deserializer};
use serde::de::{self, Visitor, MapAccess};
use std::fs;
use std::net::SocketAddr;
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::{YamlLoader, YamlEmitter};
//use system-config::new;
//use serde_yaml;


fn usage(){
  println!("USAGE: urbit-monitor [yaml config file]");
  println!("       e.g. urbit-monitor config.yaml");
  
  std::process::exit(exitcode::OK);    
}

fn err(errtxt: &str){
  println!("{} error has occurred.  Exiting.", errtxt);
  std::process::exit(exitcode::DATAERR);
}

struct admin_config {
  text_alerting_enabled: bool,
  text_alerting_endpoint: String,
  text_alerting_token: String,
  logging_enabled: bool,
  logging_file: String,
}


fn main() {
  let args: Vec<String> = env::args().collect();
  let config_file = &args[1];
  //let sdf = P { address: String::from("addy"),  code: String::from("code")};
  println!("Staring urbit monitor...");

  //validate config file
  if args.len() != 2 {
      usage();
  }else{
    //println!("Checking config: {} ", config_file);
    if Path::new(config_file).exists(){
      //println!("Config file {} exists.", config_file);
      let mut file = File::open(config_file).expect("Unable to open config.");
      let mut monitor_configs = String::new();
      file.read_to_string(&mut monitor_configs).expect("Unable to read the file");
      println!("configs: {}", monitor_configs);
      //let configs_yaml = YamlLoader::load_from_str(&monitor_configs).unwrap();

      //assert!(configs_yaml["endpoints"][0].as_str().unwrap());
      //println!("{}", configs_yaml);
      let docs = YamlLoader::load_from_str(&monitor_configs).unwrap();
      let doc = &docs[0];
      println!("{:?}", doc);
      //assert_eq!(doc["text-alerting"][0].as_str().unwrap(), "enabled");
      println!("{:?}", doc["testkey"][0].as_str().unwrap());
      println!("{:?}", doc["text-alerting"]["enabled"].as_str().unwrap());
      //assert_eq!(doc["enabled"][0].as_bool().unwrap(), false);
      //assert_eq!(doc["admin"][0].as_str().unwrap());
    }else{
      err("ERROR: Config file not found.");
    }
  }

}