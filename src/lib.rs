#![feature(test)]
extern crate test;
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::io::BufRead;
mod model;

/// ログ全体の統計情報
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogReport {
    pub total: LogTotalCount,
    pub service: HashMap<String, ServiceCount>,
}

/// ログ全体の統計情報
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogTotalCount {
    pub line: usize,
    pub message_length: usize,
    pub facility: HashMap<String, usize>,
}

/// サービスごとの統計情報
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ServiceCount {
    pub line: usize,
    pub message_length: usize,
    pub priorities: HashMap<u8, usize>,
    pub keywords: HashMap<String, usize>,
}

pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub untile: Option<DateTime<Utc>>,
}

/// line count
pub fn count(input: impl BufRead) -> Result<LogReport, serde_json::error::Error> {
    let mut counter: LogTotalCount = Default::default();
    let mut services_counter: HashMap<String, ServiceCount> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        debug!("raw line {}: {:?}", i, line);
        let p: model::Log = model::deserialize_fallback(&line)?;
        debug!("{}: {:?}", i, p);
        counter.line += 1;
        match p {
            model::Log::Kernel(l) => {
                *counter.facility.entry("kernel".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
            model::Log::Journal(l) => {
                *counter.facility.entry("journal".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();

                let mut s = services_counter.entry(l.systemd_unit).or_default();
                s.line += 1;
                s.message_length += l.message.len();
                *s.priorities.entry(l.priority).or_insert(0) += 1;
            }
            model::Log::Syslog(l) => {
                *counter.facility.entry("syslog".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
            model::Log::Stdout(l) => {
                *counter.facility.entry("stdout".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();

                let mut s = services_counter.entry(l.systemd_unit).or_default();
                s.line += 1;
                s.message_length += l.message.len();
                *s.priorities.entry(l.priority).or_insert(0) += 1;
            }
            model::Log::Audit(l) => {
                *counter.facility.entry("audit".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
            _ => {
                *counter.facility.entry("invalid".to_string()).or_insert(0) += 1;
            }
        }
    }
    Ok(LogReport {
        total: counter,
        service: services_counter,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_testfile() {
        env_logger::init();
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open("./tests/testdata/sample.log").unwrap();
        let reader = BufReader::new(&file);
        let result = count(reader).unwrap();
        println!("{:?}", result);
    }
}
