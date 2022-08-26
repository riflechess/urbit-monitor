extern crate exitcode;
extern crate system_config;
extern crate yaml_rust;

use std::env;
use std::path::Path;
use std::io::prelude::*;
use std::fs::File;
use yaml_rust::{YamlLoader};
use urbit_http_api::ShipInterface;
use std::{thread, time::Duration};
use utils::{ts, add_planet_alert};
use alerts::alerting_receiver;

mod alerts;
mod utils;

static VERSION: &str = "0.0.2";

fn usage(){
  println!("USAGE: urbitmon [yaml config file]");
  println!("       e.g. urbitmon config.yaml");
  println!("       urbitmon: {}", VERSION);
  std::process::exit(exitcode::OK);    
}

fn main() {
  let args: Vec<String> = env::args().collect();
  
  if args.len() != 2 {
    usage();
  }else{
    println!("{} - Staring urbitmon...", ts());
    let config_file = &args[1];
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
      let alert_snooze = cfg["monitoring"]["alert_snooze"].as_i64().unwrap();
      let mut alert_snooze_ct = alert_snooze;
      let mut alerting: bool = false;
      let mut alerting_planets = String::new();

      // text alerting vars
      let text_alerting_config = &cfg["text_alerting"];
      let text_alerting_enabled: bool = ! text_alerting_config.is_badvalue();

      // urbit alerting vars 
      let urbit_alerting_config = &cfg["urbit_group_alert"];
      let urbit_alerting_enabled: bool = ! urbit_alerting_config.is_badvalue();     

      // set endpoints to monitor
      let planets = cfg["endpoints"].as_hash().unwrap();
      loop{
        // reset alerting planets 
        alerting_planets.clear();

        for planet in planets {
          let planet_name = planet.0.as_str().unwrap();
          let planet_address = cfg["endpoints"][planet.0.as_str().unwrap()]["address"].as_str().unwrap();
          let planet_code = cfg["endpoints"][planet.0.as_str().unwrap()]["code"].as_str().unwrap();
          println!("{} - Checking: {} ({})", ts(), planet_name, planet_address);
          
          // simple login check - don't unwrap 
          let ship_interface = ShipInterface::new(planet_address, planet_code);
          
          if ship_interface.is_ok() {
            println!("{} - {} [OK]", ts(), planet_name);
          } else {
            println!("{} - {} [ERROR] Login failed.", ts(), planet_name);
            alerting = true;
            add_planet_alert(&mut alerting_planets, planet_name);
          }
        }
        if alerting {
          // if service mode && counter = snooze
          if !service_mode {
            if text_alerting_enabled {
              alerting_receiver(&alerting_planets, "text_alert", text_alerting_config);
            }
            if urbit_alerting_enabled {
              alerting_receiver(&alerting_planets, "urbit_alert", urbit_alerting_config); 
            }
            println!("{} - Exiting urbitmon...", ts());
            break;
          }else{
            // service mode alerting with snooze
            // first alert send decrement snooze count
            if alert_snooze_ct == alert_snooze {
              if text_alerting_enabled{
                alerting_receiver(&alerting_planets, "text_alert", text_alerting_config);
              }
              if urbit_alerting_enabled {
                alerting_receiver(&alerting_planets, "urbit_alert", urbit_alerting_config); 
              }
              alert_snooze_ct = alert_snooze_ct - 1;
            // snooze ends, send alert, reset snooze
            }else if alert_snooze_ct == 1{
              println!("{} - Snooze ended, enabling alerts.", ts());
              // alerting_receiver(&alerting_planets, "text_alert", text_alerting_config);
              alert_snooze_ct = alert_snooze;
            // snooze zone, don't alert, decrement snooze count
            }else{
              println!("{} - Snoozing alerts.", ts());
              alert_snooze_ct = alert_snooze_ct - 1;       
            }
            // sleep for monitoring interval, reset alert
            alerting = false;
            thread::sleep(Duration::from_secs(monitoring_interval.try_into().unwrap()));            
          }
        }else{
          if !service_mode {
            println!("{} - Exiting urbitmon...", ts());
            break;
          }else{
            thread::sleep(Duration::from_secs(monitoring_interval.try_into().unwrap()));   
          }
        }
      }
    }else{
      println!("Config file {} not found. Exiting.", config_file);
      std::process::exit(exitcode::DATAERR);
    }
  }
}