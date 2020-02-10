#![feature(test)]
extern crate test;
use chrono::{DateTime, Utc};

use std::collections::HashMap;
use std::fmt;
use std::io::BufRead;
mod model;

/// ログ全体の統計情報
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogReport {
    pub total: LogTotalCount,
    pub service: HashMap<String, ServiceCount>,
}

impl fmt::Display for LogReport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "total:\n{}", &self.total)?;
        write!(f, "service:\n")?;
        let mut list = Vec::new();
        for (name, srv) in &self.service {
            list.push((name, srv))
        }
        list.sort_by(|(_name_a, srv_a), (_name_b, srv_b)| return srv_a.compare_count(srv_b));
        for (name, srv) in &list {
            writeln!(f, "- {}\n{}", name, srv)?;
        }
        Ok(())
    }
}
/// ログ全体の統計情報
#[derive(Debug, Clone, PartialEq, Default)]
pub struct LogTotalCount {
    pub line: usize,
    pub message_length: usize,
    pub facility: HashMap<String, usize>,
}

impl fmt::Display for LogTotalCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  line: {}\n  message_length:{}\n",
            &self.line, &self.message_length
        )?;
        write!(f, "facility:\n")?;
        for (name, count) in &self.facility {
            writeln!(f, "  {}: {}", name, count)?;
        }
        Ok(())
    }
}

/// サービスごとの統計情報
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ServiceCount {
    pub line: usize,
    pub message_length: usize,
    pub priorities: HashMap<u8, usize>,
    pub keywords: HashMap<String, usize>,
}

impl ServiceCount {
    fn total_line_count(&self) -> usize {
        let mut count = 0;
        for (_, n) in &self.priorities {
            count += n;
        }
        count
    }

    fn compare_count(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.total_line_count();
        let b = other.total_line_count();
        if a < b {
            std::cmp::Ordering::Less
        } else if a > b {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Equal
        }
    }
}

impl fmt::Display for ServiceCount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "  - line: {}\n  - length:{}\n  - priority:{:?}",
            &self.line, &self.message_length, &self.priorities
        )
    }
}

pub struct DateTimeRange {
    pub from: Option<DateTime<Utc>>,
    pub untile: Option<DateTime<Utc>>,
}

/// line count
pub fn count(input: impl BufRead) -> Result<LogReport, serde_json::error::Error> {
    let mut counter: LogTotalCount = Default::default();
    let mut services_counter: HashMap<String, ServiceCount> = HashMap::new();
    for (_i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        // debug!("raw line {}: {:?}", i, line);
        let p: model::Log = model::deserialize_fallback(&line)?;
        // debug!("{}: {:?}", i, p);
        counter.line += 1;
        match p {
            model::Log::Kernel(l) => {
                *counter.facility.entry("kernel".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
            model::Log::Journal(l) => {
                *counter.facility.entry("journal".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();

                let mut s = services_counter
                    .entry(l.systemd_unit.to_string())
                    .or_default();
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

                let mut s = services_counter
                    .entry(l.systemd_unit.to_string())
                    .or_default();
                s.line += 1;
                s.message_length += l.message.len();
                *s.priorities.entry(l.priority).or_insert(0) += 1;
            }
            model::Log::Audit(l) => {
                *counter.facility.entry("audit".to_string()).or_insert(0) += 1;
                counter.message_length += l.message.len();
            }
            model::Log::Driver(l) => {
                *counter.facility.entry("driver".to_string()).or_insert(0) += 1;
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
        println!("{}", result);
    }
    #[test]
    fn service_count_order() {
        let s1 = ServiceCount {
            line: 1,
            priorities: [(b'4', 1)].iter().cloned().collect::<HashMap<u8, usize>>(),
            ..Default::default()
        };
        let s2 = ServiceCount {
            line: 10,
            priorities: [(b'4', 10)].iter().cloned().collect::<HashMap<u8, usize>>(),
            ..Default::default()
        };

        assert_eq!(s1.compare_count(&s2), std::cmp::Ordering::Less);
        assert_eq!(s1.compare_count(&s1), std::cmp::Ordering::Equal);
        assert_eq!(s2.compare_count(&s1), std::cmp::Ordering::Greater);
    }
}
