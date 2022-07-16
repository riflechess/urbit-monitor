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


fn main() {
  let args: Vec<String> = env::args().collect();
  let configFile = &args[1];
  //error if not one arg
  if args.len() != 2 {
      usage();
  }else{
    let config = &args[1];
    println!("Found config file:{} ", configFile);
    println!("{}", Path::new(configFile).exists());
  }

  //let chat_bot = Chatbot::new_with_local_config(respond_to_message, "~sitsev-lomrem", "chat-6301");
  println!("Staring urbit monitor.");
}