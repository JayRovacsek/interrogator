use super::log::Log;
use crate::log_options::LogOptions;
use crate::ProgramOptions;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Ingestor {
    pub file_name: String,
    log_type: String,
    program_options: ProgramOptions,
    re: (Regex, Regex),
}

impl std::fmt::Display for Ingestor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Options: {:?}
            Regex: {:?}",
            self.program_options, self.re
        )
    }
}

impl Ingestor {
    pub fn new(option: u8, log_options: LogOptions, program_options: ProgramOptions) -> Ingestor {
        Ingestor {
            file_name: String::new(),
            log_type: log_options.options.get(&option).unwrap().to_string(),
            program_options,
            re: select_regex(option),
        }
    }

    pub fn ingest_file(&mut self, file_name: &str) -> Vec<Log> {
        self.file_name = file_name.clone().to_string();

        let file = BufReader::new(File::open(&self.file_name).unwrap());
        let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
        if self.program_options.verbose {
            println!("Number of lines in {:?}: {:?}", self.file_name, lines.len());
        }

        match self.program_options.sequential {
            false => lines
                .par_iter()
                .map(|line| super::log::parse(&line, &self.re))
                .collect(),
            _ => lines
                .iter()
                .map(|line| super::log::parse(&line, &self.re))
                .collect(),
        }
    }
}

fn select_regex(option: u8) -> (Regex, Regex) {
    match option {
        0 => (regex::Regex::new(
                r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})\] "(\S+) (\S+)\s*(\S+)?\s*" (\d{3}) (\S+) ("\S+") (.*)"#,
        ).unwrap(),
        regex::Regex::new(
                r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})\] "(\S+) (\S+)\s*(\S+)?\s* (\S+) (\S+)"#,
        ).unwrap())
        ,
        _ => panic!("Could not find an appropriate regex")
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     // #[test]
//     // fn test_select_regex() {
//     //     assert_eq!(select_regex(0),regex::Regex::new(r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})\] "(\S+) (\S+)\s*(\S+)?\s*" (\d{3}) (\S+) ("\S+") (.*)"#).unwrap());
//     //     assert_eq!(select_regex(0),regex::Regex::new("").unwrap());
//     //     assert_ne!(select_regex(0),regex::Regex::new("").unwrap());
//     //     assert_ne!(select_regex(255),regex::Regex::new(r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})\] "(\S+) (\S+)\s*(\S+)?\s*" (\d{3}) (\S+) ("\S+") (.*)"#).unwrap());
//     // }

//     #[test]
//     #[should_panic]
//     fn test_parse_status_panic() {
//         assert_eq!("-1".to_owned().parse::<u16>().unwrap(), 1 as u16)
//     }
// }
