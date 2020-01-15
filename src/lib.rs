#![feature(test)]
extern crate test;
use log::{debug, error, info, warn};
use std::io::BufRead;
mod model;

/// line count
pub fn count(input: impl BufRead) {
    for (i, line) in input.lines().enumerate() {
        let line = line.unwrap();
        debug!("raw line {}: {:?}", i, line);
        let p: model::Log = serde_json::from_str(&line).unwrap();
        debug!("{}: {:?}", i, p);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn read_testfile() {
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open("./tests/testdata/sample.log").unwrap();
        let reader = BufReader::new(&file);
        count(reader)
    }
}
