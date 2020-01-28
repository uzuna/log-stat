extern crate clap;
extern crate log_stat;
use clap::{crate_name, crate_version, App, Arg, SubCommand};
use log::debug;
use log_stat::*;
use std::error::Error;
use std::io;
use std::io::prelude::*;
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
        .takes_value(true),
    )
    .get_matches();

  let cmd = matches.value_of("cmd").unwrap_or("sudo journalclt -o json");
  let mut ary = cmd.split_whitespace();
  let base_cmd = ary.nth(0).unwrap();
  let args = ary.collect::<Vec<&str>>();
  debug!("{} {:?}", base_cmd, args);
  let process = match Command::new(base_cmd)
    .args(args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
  {
    Err(why) => panic!("couldn't spawn wc: {}", Error::description(&why)),
    Ok(process) => process,
  };

  let reader = BufReader::new(process.stdout.unwrap());
  let result = count(reader).unwrap();
  println!("{}", result);
}
