//use urbit-http-api::{};

extern crate exitcode;

use std::io::Read;
use std::env;
use std::path::Path;

fn usage(){
  println!("USAGE: urbit-monitor [config file]");
  println!("       e.g. urbit-monitor config");
  
  std::process::exit(exitcode::OK);    
}

fn err(errtxt: &str){
  println!("{} error has occurred.  Exiting.", errtxt);
  std::process::exit(exitcode::DATAERR);
}

fn main() {
  let args: Vec<String> = env::args().collect();
  let configFile = &args[1];
  println!("Staring urbit monitor...");

  //validate config file
  if args.len() != 2 {
      usage();
  }else{
    let config = &args[1];
    println!("Checking config: {} ", configFile);
    if Path::new(configFile).exists(){
      println!("Config file exists.");
    }else{
      err("ERROR: Config file not found.");
    }
  }

}