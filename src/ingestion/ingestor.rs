use super::log::Log;
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
    pub fn new(
        option: u8,
        options: HashMap<u8, &str>,
        program_options: ProgramOptions,
    ) -> Ingestor {
        Ingestor {
            file_name: String::new(),
            log_type: options.get(&option).unwrap().to_string(),
            program_options: program_options,
            re: select_regex(option),
        }
    }

    pub fn ingest_file(&mut self, file_name: &str) -> Vec<Log> {
        self.file_name = file_name.clone().to_string();
        let mut logs: Vec<Log> = Vec::new();

        let file = BufReader::new(File::open(&self.file_name).unwrap());
        let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();
        if self.program_options.verbose {
            println!("Number of lines: {:?}", lines.len());
        }

        match self.program_options.sequential {
            true => lines
                .par_iter()
                .map(|line| super::log::parse(&line, &self.re))
                .collect_into_vec(&mut logs),
            false => lines
                .par_iter()
                .map(|line| super::log::parse(&line, &self.re))
                .collect_into_vec(&mut logs),
        }
        logs
    }
}

fn select_regex(option: u8) -> (Regex, Regex) {
    let empty_regex: Regex = regex::Regex::new("").unwrap();
    match option {
        0 => (regex::Regex::new(
                r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})\] "(\S+) (\S+)\s*(\S+)?\s*" (\d{3}) (\S+) ("\S+") (.*)"#,
        ).unwrap(),
        regex::Regex::new(
                r#"^(\S+) (\S+) (\S+) \[([\w:/]+\s[+\-]\d{4})\] "(\S+) (\S+)\s*(\S+)?\s* (\S+) (\S+)"#,
        ).unwrap())
        ,
        _ => (empty_regex.clone(), empty_regex)
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
