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
      let cfgs = YamlLoader::load_from_str(&monitor_configs).unwrap();
      let cfg = &cfgs[0];
      println!("{:?}", cfg);
      //assert_eq!(cfg["text-alerting"][0].as_str().unwrap(), "enabled");
      //println!("{:?}", cfg["testkey"][0].as_str().unwrap());
      // text-alerting vars 
      println!("{:?}", cfg["text-alerting"]["enabled"].as_bool().unwrap());
      let do_alerting = cfg["text-alerting"]["enabled"].as_bool().unwrap();
      let alerting_endpoint = cfg["text-alerting"]["endpoint"].as_str().unwrap();
      let alerting_token = cfg["text-alerting"]["token"].as_str().unwrap();
      println!("Alerting vars - enabled:{}, endpoint:{}, token:{}", do_alerting, alerting_endpoint, alerting_token);
      //assert_eq!(cfg["enabled"][0].as_bool().unwrap(), false);
      //assert_eq!(cfg["admin"][0].as_str().unwrap());

      let planets = cfg["endpoints"].as_hash().unwrap();
      for planet in planets {
        println!("Checking {:?}.",planet.0.as_str().unwrap()); 
        let planet_address = cfg["endpoints"][planet.0.as_str().unwrap()]["address"].as_str().unwrap();
        let planet_code = cfg["endpoints"][planet.0.as_str().unwrap()]["code"].as_str().unwrap();
        println!("  Planet address: {}",planet_address);
        println!("  Planet code: {}",planet_code);        
      }
      //println!("zzzz:{}",xxx);
      //println!("{:?}", xxx);
      //for planet in cfg["endpoints"].as_vec().unwrap() {
      //  println!("hi");
      //}
      // planet vars
      //println!("check:{}",cfg["endpoints"]["planet1"].contains_key());

      //let planet_name = cfg["endpoints"]["planet1"]["name"].as_str().unwrap();
      //let planet_address = cfg["endpoints"]["planet1"]["address"].as_str().unwrap();
      //let planet_code = cfg["endpoints"]["planet1"]["code"].as_str().unwrap();      
      //println!("Planet vars - name:{}, address:{}", planet_name, planet_address);
            

      
    }else{
      err("ERROR: Config file not found.");
    }
  }

}