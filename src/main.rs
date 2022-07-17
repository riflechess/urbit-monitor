#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

extern crate exitcode;
extern crate system_config;
extern crate yaml_rust;

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
use urbit_http_api::ShipInterface;
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

fn main() {
  let args: Vec<String> = env::args().collect();
  let config_file = &args[1];
  println!("Staring urbit monitor...");

  if args.len() != 2 {
      usage();
  }else{
    if Path::new(config_file).exists(){
      // open and read config.yml
      let mut file = File::open(config_file).expect("Unable to open config.");
      let mut monitor_configs = String::new();
      file.read_to_string(&mut monitor_configs).expect("Unable to read the file");
      println!("configs: {}", monitor_configs);

      // parse yaml
      let cfgs = YamlLoader::load_from_str(&monitor_configs).unwrap();
      let cfg = &cfgs[0];

      // text-alerting vars 
      println!("{:?}", cfg["text-alerting"]["enabled"].as_bool().unwrap());
      let do_alerting = cfg["text-alerting"]["enabled"].as_bool().unwrap();
      let alerting_endpoint = cfg["text-alerting"]["endpoint"].as_str().unwrap();
      let alerting_token = cfg["text-alerting"]["token"].as_str().unwrap();
      println!("Alerting vars - enabled:{}, endpoint:{}, token:{}", do_alerting, alerting_endpoint, alerting_token);

      let planets = cfg["endpoints"].as_hash().unwrap();
      for planet in planets {
        let planet_name = planet.0.as_str().unwrap();
        let planet_address = cfg["endpoints"][planet.0.as_str().unwrap()]["address"].as_str().unwrap();
        let planet_code = cfg["endpoints"][planet.0.as_str().unwrap()]["code"].as_str().unwrap();
        println!("Checking {}", planet_name);
        println!("  Planet address: {}",planet_address);
        let mut planet_code_trunc = String::from(planet_code);
        planet_code_trunc.truncate(2);
        println!("  Planet code: {}xxxx-xxxxxx-xxxxxx-xxxxxx",planet_code_trunc);        
      }



    }else{
      err("ERROR: Config file not found.");
    }
  }

}