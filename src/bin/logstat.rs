extern crate clap;
extern crate log_stat;
use clap::{crate_name, crate_version, App, Arg};
use log::debug;
use log_stat::*;
use std::error::Error;

use std::io::BufReader;
use std::process::{Command, Stdio};

fn main() {
  env_logger::init();

  let matches = App::new(crate_name!())
    .version(crate_version!())
    .arg(
      Arg::with_name("cmd")
        .long("cmd")
        .value_name("ARGS")
        .help("Set using command")
        .default_value("sudo journalclt -o json")
        .takes_value(true),
    )
    .get_matches();

  let cmd = matches.value_of("cmd").unwrap();
  let mut ary = cmd.split_whitespace();
  let base_cmd = ary.nth(0).unwrap();
  let args = ary.collect::<Vec<&str>>();
  debug!("{} {:?}", base_cmd, &args);
  let process = match Command::new(base_cmd)
    .args(&args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
  {
    Err(why) => panic!("couldn't spawn wc: {}", Error::description(&why)),
    Ok(process) => process,
  };

  let reader = BufReader::new(process.stdout.unwrap());
  match count(reader) {
    Ok(r) => {
      println!("{}", r);
    }
    Err(e) => {
      // Err();
      eprintln!("parse error [{}] by cmd: {} {:?}", e, base_cmd, &args);
      std::process::exit(1);
    }
  };
}
