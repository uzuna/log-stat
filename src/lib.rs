#![feature(test)]
extern crate test;
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use std::collections::HashMap;
use std::io::BufRead;
mod model;

#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogTotalCount {
    pub line: usize,
    pub message_length: usize,
    pub facility: HashMap<String, usize>,
}

pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub untile: Option<DateTime<Utc>>,
}

/// line count
pub fn count(input: impl BufRead) -> Result<LogTotalCount, serde_json::error::Error> {
    let mut counter: LogTotalCount = Default::default();
    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        debug!("raw line {}: {:?}", i, line);
        let p: model::Log = serde_json::from_str(&line)?;
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
            }
            model::Log::Syslog(l) => {
                *counter.facility.entry("syslog".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
            model::Log::Stdout(l) => {
                *counter.facility.entry("stdout".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
            model::Log::Audit(l) => {
                *counter.facility.entry("audit".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
        }
    }
    Ok(counter)
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
