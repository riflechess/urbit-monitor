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
use std::{thread, time::Duration};

//use system-config::new;
//use serde_yaml;


fn usage(){
  println!("USAGE: urbitmon [yaml config file]");
  println!("       e.g. urbitmon config.yaml");
  
  std::process::exit(exitcode::OK);    
}

fn err(errtxt: &str){
  println!("{} error has occurred.  Exiting.", errtxt);
  std::process::exit(exitcode::DATAERR);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let config_file = &args[1];
  println!("Staring urbitmon...");

  if args.len() != 2 {
      usage();
  }else{
    if Path::new(config_file).exists(){
      // open and read config.yml
      let mut file = File::open(config_file).expect("Unable to open config.");
      let mut monitor_configs = String::new();
      file.read_to_string(&mut monitor_configs).expect("Unable to read the file");
      

      // parse yaml config file
      let cfgs = YamlLoader::load_from_str(&monitor_configs).unwrap();
      let cfg = &cfgs[0];

      // set monitoring vars 
      let monitoring_interval = cfg["monitoring"]["interval"].as_i64().unwrap();
      let service_mode: bool = monitoring_interval != 0;

      // set text-alerting vars 
      if cfg["text-alerting"]["enabled"].as_bool().unwrap() {
        let do_alerting = cfg["text-alerting"]["enabled"].as_bool().unwrap();
        let alerting_endpoint = cfg["text-alerting"]["endpoint"].as_str().expect("Alerting endpoint not defined");
        let alerting_token = cfg["text-alerting"]["token"].as_str().expect("Alerting token not defined");
        println!("Alerting vars - enabled:{}, endpoint:{}, token:{}", do_alerting, alerting_endpoint, alerting_token);  
      }

      let planets = cfg["endpoints"].as_hash().unwrap();
      loop{
        for planet in planets {
          let planet_name = planet.0.as_str().unwrap();
          let planet_address = cfg["endpoints"][planet.0.as_str().unwrap()]["address"].as_str().unwrap();
          let planet_code = cfg["endpoints"][planet.0.as_str().unwrap()]["code"].as_str().unwrap();
          println!("Checking: {}, address: {}", planet_name, planet_address);
          
          // simple login check - don't unwrap 
          let ship_interface = ShipInterface::new(planet_address, planet_code);
          
          if ship_interface.is_ok() {
            println!("  Login status: {}", ship_interface.is_ok());
          } else {
            println!("  Login status: {}", ship_interface.is_ok());
            // call alert here 
          }
        }
        // wait if in service mode, exit if not
        if service_mode {
          let seconds: u64 = monitoring_interval.try_into().unwrap();
          thread::sleep(Duration::from_secs(monitoring_interval.try_into().unwrap()));          
        }else{
          println!("Exiting urbitmon...");
          break;
        }
      }



    }else{
      err("ERROR: Config file not found.");
    }
  }

}