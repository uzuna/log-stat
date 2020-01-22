use chrono::offset::TimeZone;
use chrono::{DateTime, NaiveDateTime, Utc};
use failure::Error;
use log::{debug, error, info, warn};
use serde::{Deserialize, Deserializer, Serialize};
use serde_derive::*;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "_TRANSPORT")]
pub enum Log {
  #[serde(rename = "journal")]
  Journal(Journal),
  #[serde(rename = "kernel")]
  Kernel(Kernel),
  #[serde(rename = "stdout")]
  Stdout(Stdout),
  #[serde(rename = "audit")]
  Audit(Audit),
  #[serde(rename = "syslog")]
  Syslog(Syslog),
  Invalid(Invalid),
}

/// Kernelログ
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Journal {
  #[serde(rename(deserialize = "_PID"), deserialize_with = "from_str")]
  pub pid: u16,

  // pub bootid: [u8; 16],
  #[serde(
    rename(deserialize = "PRIORITY"),
    deserialize_with = "from_str",
    default
  )]
  pub priority: u8,

  #[serde(
    rename(deserialize = "_SYSTEMD_UNIT"),
    default = "default_systemd_unit"
  )]
  pub systemd_unit: String,

  #[serde(rename(deserialize = "MESSAGE"))]
  pub message: String,

  #[serde(
    rename(deserialize = "__REALTIME_TIMESTAMP"),
    deserialize_with = "datefmt"
  )]
  pub realtime_timestamp: DateTime<Utc>,

  #[serde(
    rename(deserialize = "__MONOTONIC_TIMESTAMP"),
    deserialize_with = "from_str"
  )]
  pub monotonic_timestamp: u64,
}
/// Kernelログ
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Kernel {
  #[serde(rename(deserialize = "SYSLOG_IDENTIFIER"))]
  pub identifier: String,

  #[serde(rename(deserialize = "PRIORITY"), deserialize_with = "from_str")]
  pub priority: u8,

  #[serde(rename(deserialize = "MESSAGE"))]
  pub message: String,

  #[serde(
    rename(deserialize = "__REALTIME_TIMESTAMP"),
    deserialize_with = "datefmt"
  )]
  pub realtime_timestamp: DateTime<Utc>,

  #[serde(
    rename(deserialize = "__MONOTONIC_TIMESTAMP"),
    deserialize_with = "from_str"
  )]
  pub monotonic_timestamp: u64,
}

/// Stdoutログ
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Stdout {
  #[serde(rename(deserialize = "SYSLOG_IDENTIFIER"))]
  pub identifier: String,

  #[serde(
    rename(deserialize = "_SYSTEMD_UNIT"),
    default = "default_systemd_unit"
  )]
  pub systemd_unit: String,

  #[serde(rename(deserialize = "PRIORITY"), deserialize_with = "from_str")]
  pub priority: u8,

  #[serde(rename(deserialize = "MESSAGE"))]
  pub message: String,

  #[serde(
    rename(deserialize = "__REALTIME_TIMESTAMP"),
    deserialize_with = "datefmt"
  )]
  pub realtime_timestamp: DateTime<Utc>,

  #[serde(
    rename(deserialize = "__MONOTONIC_TIMESTAMP"),
    deserialize_with = "from_str"
  )]
  pub monotonic_timestamp: u64,
}

/// auditログ
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Audit {
  #[serde(rename(deserialize = "SYSLOG_IDENTIFIER"))]
  pub identifier: String,

  #[serde(
    rename(deserialize = "PRIORITY"),
    deserialize_with = "from_str",
    default
  )]
  pub priority: u8,

  #[serde(rename(deserialize = "MESSAGE"))]
  pub message: String,

  #[serde(
    rename(deserialize = "__REALTIME_TIMESTAMP"),
    deserialize_with = "datefmt"
  )]
  pub realtime_timestamp: DateTime<Utc>,

  #[serde(
    rename(deserialize = "__MONOTONIC_TIMESTAMP"),
    deserialize_with = "from_str"
  )]
  pub monotonic_timestamp: u64,
}

/// syslogログ
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Syslog {
  #[serde(rename(deserialize = "SYSLOG_IDENTIFIER"))]
  pub identifier: String,

  #[serde(rename(deserialize = "PRIORITY"), deserialize_with = "from_str")]
  pub priority: u8,

  #[serde(rename(deserialize = "MESSAGE"))]
  pub message: String,

  #[serde(
    rename(deserialize = "__REALTIME_TIMESTAMP"),
    deserialize_with = "datefmt"
  )]
  pub realtime_timestamp: DateTime<Utc>,

  #[serde(
    rename(deserialize = "__MONOTONIC_TIMESTAMP"),
    deserialize_with = "from_str"
  )]
  pub monotonic_timestamp: u64,
}

/// Messageが不正なデータなどの場合
#[derive(Debug, Clone, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub struct Invalid {
  #[serde(rename(deserialize = "SYSLOG_IDENTIFIER"))]
  pub identifier: String,

  #[serde(
    rename(deserialize = "PRIORITY"),
    deserialize_with = "from_str",
    default
  )]
  pub priority: u8,

  #[serde(skip_deserializing)]
  pub error: String,

  #[serde(
    rename(deserialize = "__REALTIME_TIMESTAMP"),
    deserialize_with = "datefmt"
  )]
  pub realtime_timestamp: DateTime<Utc>,

  #[serde(
    rename(deserialize = "__MONOTONIC_TIMESTAMP"),
    deserialize_with = "from_str"
  )]
  pub monotonic_timestamp: u64,
}

/// jsonで文字列化されている型を適切に変換
fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
  T: FromStr,
  T::Err: Display,
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  T::from_str(&s).map_err(serde::de::Error::custom)
}

/// unixtime_microsecをDatetime<Utc>に変換
fn datefmt<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
  D: Deserializer<'de>,
{
  let s = String::deserialize(deserializer)?;
  let s = i64::from_str(&s).unwrap();
  let ts = NaiveDateTime::from_timestamp(s / 1000000, (s as u32 % 1000000) * 1000);
  Ok(Utc.from_utc_datetime(&ts))
}

fn default_systemd_unit() -> String {
  "unknwon".to_string()
}

/// 意図しないフォーマットや項目の不足がまだあるため
/// 想定外のデータはInvalidType型にして返す
pub fn deserialize_fallback(log: &str) -> Result<Log, serde_json::error::Error> {
  match serde_json::from_str::<Log>(log) {
    Ok(p) => Ok(p),
    Err(e) => {
      let mut p = serde_json::from_str::<Invalid>(log)?;
      p.error = e.to_string();
      warn!("deserialize: {:?}", e);
      debug!("deserialize error at {}", log);
      Ok(Log::Invalid(p))
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;
  #[test]
  fn it_works() -> Result<(), Error> {
    let testdata_log = r#"{
    "__CURSOR" : "s=46ba892f70824638b3d16da7d80834ac;i=a4f10;b=19b1f03dcfef43179df0082144e9f33a;m=44598cf;t=54c69409a0b92;x=cb2f553f676a9051",
    "__REALTIME_TIMESTAMP" : "1491389822667666",
    "__MONOTONIC_TIMESTAMP" : "71669967",
    "_BOOT_ID" : "19b1f03dcfef43179df0082144e9f33a",
    "_TRANSPORT" : "journal",
    "_MACHINE_ID" : "1ddaa2b4178f429b82a894ea90b7e7d0",
    "_HOSTNAME" : "master",
    "PRIORITY" : "6",
    "SYSLOG_FACILITY" : "3",
    "_SELINUX_CONTEXT" : "system_u:system_r:init_t:s0",
    "_UID" : "0",
    "_GID" : "0",
    "_CAP_EFFECTIVE" : "1fffffffff",
    "CODE_FILE" : "src/core/job.c",
    "CODE_LINE" : "776",
    "CODE_FUNCTION" : "job_log_status_message",
    "SYSLOG_IDENTIFIER" : "systemd",
    "MESSAGE_ID" : "39f53479d3a045ac8e11786248231fbf",
    "RESULT" : "done",
    "_PID" : "1",
    "_COMM" : "systemd",
    "_EXE" : "/usr/lib/systemd/systemd",
    "_CMDLINE" : "/usr/lib/systemd/systemd --switched-root --system --deserialize 21",
    "_SYSTEMD_CGROUP" : "/",
    "_SYSTEMD_UNIT" : "docker.service",
    "MESSAGE" : "Started Docker Application Container Engine.",
    "_SOURCE_REALTIME_TIMESTAMP" : "1491389822657954"
}"#;
    let p: Log = serde_json::from_str(testdata_log)?;
    println!("{:?}", p);
    Ok(())
  }
  #[test]
  fn parse_bytearray() -> Result<(), Error> {
    let testdata_log = r#"{
      "SYSLOG_IDENTIFIER" : "systemd",
      "__REALTIME_TIMESTAMP" : "1491389822667666",
      "__MONOTONIC_TIMESTAMP" : "71669967",
      "PRIORITY" : "6",
      "MESSAGE" : [27,91,51,54,109,97,112,112,95,49,32,32,124,27,91,48,109,32,50,48,50,48,45,48,49,45,49,53,32,48,51,58,49,54,58,50,51,32,115,112,105,107,101,32,108,111,118,111,116,95,115,108,97,109,46,115,108,97,109,95,115,101,114,118,105,99,101,114,91,54,54,93,32,73,78,70,79,32,103,101,116,32,115,112,111,116,115,58,32,91,39,101,110,116,114,97,110,99,101,39,93]
}"#;

    let p = deserialize_fallback(&testdata_log).unwrap();
    println!("parse_bytearray: {:?}", p);
    Ok(())
  }

  // ベンチマークの結果i64でパースするほうがstringで行うより早かった
  static UNIX_MICRO_SECOND_EXAMPLE: &'static str = "1491389822667666";
  #[bench]
  fn bench_parse_i64(b: &mut Bencher) {
    use chrono::offset::TimeZone;
    use chrono::{DateTime, NaiveDateTime, Utc};
    fn parse(s: String) -> DateTime<Utc> {
      let s = i64::from_str(&s).unwrap();
      let ts = NaiveDateTime::from_timestamp(s / 1000000, (s as u32 % 1000000) * 1000);
      Utc.from_utc_datetime(&ts)
    }
    b.iter(|| parse(String::from(UNIX_MICRO_SECOND_EXAMPLE)));
  }
  #[bench]
  fn bench_parse_string(b: &mut Bencher) {
    use chrono::offset::TimeZone;
    use chrono::{DateTime, NaiveDateTime, Utc};
    fn parse(s: String) -> DateTime<Utc> {
      let ts = NaiveDateTime::parse_from_str(&s[0..s.len() - 6], "%s").unwrap();
      Utc.from_utc_datetime(&ts)
    }
    b.iter(|| parse(String::from(UNIX_MICRO_SECOND_EXAMPLE)));
  }
}