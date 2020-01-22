extern crate log_stat;
use log_stat::*;
use std::error::Error;
use std::io::prelude::*;
use std::io::BufReader;
use std::process::{Command, Stdio};

fn main() {
  let process = match Command::new("sudo")
    .args(&["journalctl", "-o", "json"])
    .stdout(Stdio::piped())
    .spawn()
  {
    Err(why) => panic!("couldn't spawn wc: {}", Error::description(&why)),
    Ok(process) => process,
  };

  let reader = BufReader::new(process.stdout.unwrap());
  let result = count(reader).unwrap();
  println!("{}", result);
}
