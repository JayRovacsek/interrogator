use chrono::{DateTime, FixedOffset};
use regex::Captures;
use regex::Regex;
use std::net::IpAddr;
use std::net::Ipv4Addr;
use std::net::Ipv6Addr;

#[derive(Debug, Clone)]
pub struct Log {
    pub ip: Option<IpAddr>,
    pub remote_log_name: Option<String>,
    pub user_id: Option<String>,
    pub date: Option<DateTime<FixedOffset>>,
    pub timezone: Option<FixedOffset>,
    pub request_method: Option<Method>,
    pub path: Option<String>,
    pub status: Option<u16>,
    pub length: Option<String>,
    pub referrer: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    Options,
    Get,
    Post,
    Put,
    Delete,
    Head,
    Trace,
    Connect,
    Patch,
    Extension(String),
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}

pub fn parse(input: &str, re: &(Regex, Regex)) -> Log {
    match re.0.captures(input) {
        Some(c) => {
            // println!("Parsed line: {:?}", c);
            Log::from_apache_main_capture(&c)
        }
        None => match re.1.captures(input) {
            Some(c) => Log::from_apache_alternate_capture(&c),
            _ => panic!("Failed to parse input: {}", input),
        },
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ip: {:?}
remote_log_name: {:?}
user_ip: {:?}
date: {:?}
timezone: {:?}
request_method: {:?}
path: {:?}
status: {:?}
length: {:?}
referrer: {:?}
user_agent: {:?}",
            self.ip,
            self.remote_log_name,
            self.user_id,
            self.date,
            self.timezone,
            self.request_method,
            self.path,
            self.status,
            self.length,
            self.referrer,
            self.user_agent
        )
    }
}

impl Log {
    pub fn from_apache_main_capture(capture: &Captures) -> Log {
        Log {
            ip: parse_ip(&capture[1]),
            remote_log_name: Some(capture[2].to_string()),
            user_id: Some(capture[3].to_string()),
            date: Some(DateTime::parse_from_str(&capture[4], "%d/%b/%Y:%H:%M:%S %z").unwrap()),
            timezone: Some(
                DateTime::parse_from_str(&capture[4], "%d/%b/%Y:%H:%M:%S %z")
                    .unwrap()
                    .timezone(),
            ),
            request_method: Some(Method::Extension(
                capture[6].to_ascii_uppercase().to_string(),
            )),
            path: Some(capture[7].to_string()),
            status: parse_status(&capture[8].to_string()),
            length: Some(capture[9].to_string()),
            referrer: Some(capture[10].to_string()),
            user_agent: Some(capture[11].to_string()),
        }
    }

    pub fn exists_in_vec(log: &Log, logs: &Vec<Log>) -> bool {
        logs.contains(log)
    }

    pub fn from_apache_alternate_capture(capture: &Captures) -> Log {
        Log {
            ip: parse_ip(&capture[1]),
            remote_log_name: Some(capture[2].to_string()),
            user_id: Some(capture[3].to_string()),
            date: Some(DateTime::parse_from_str(&capture[4], "%d/%b/%Y:%H:%M:%S %z").unwrap()),
            timezone: Some(
                DateTime::parse_from_str(&capture[4], "%d/%b/%Y:%H:%M:%S %z")
                    .unwrap()
                    .timezone(),
            ),
            request_method: None,
            path: None,
            status: parse_status(&capture[6].to_string()),
            length: Some(capture[7].to_string()),
            referrer: None,
            user_agent: None,
        }
    }

    pub fn get_user(&self) -> String {
        match &self.user_id {
            Some(u) => String::from(u),
            None => String::from(""),
        }
    }
}

impl PartialEq for Log {
    fn eq(&self, other: &Self) -> bool {
        self.ip == other.ip
            && self.length == other.length
            && self.path == other.length
            && self.referrer == other.referrer
            && self.remote_log_name == other.remote_log_name
            && self.request_method == other.request_method
            && self.status == other.status
            && self.timezone == other.timezone
            && self.user_agent == other.user_agent
            && self.user_id == other.user_id
    }
}

pub fn parse_status(input: &str) -> Option<u16> {
    match input.is_empty() {
        true => Some(0 as u16),
        false => Some(input.to_owned().parse::<u16>().unwrap()),
    }
}

fn parse_ip(input: &str) -> Option<IpAddr> {
    match input.contains(".") {
        true => {
            let components: Vec<&str> = input.split(".").collect();
            Some(IpAddr::V4(Ipv4Addr::new(
                parse_u8(components[0]),
                parse_u8(components[1]),
                parse_u8(components[2]),
                parse_u8(components[3]),
            )))
        }
        _ => match input.contains("::") {
            true => {
                let components: Vec<&str> = input.split("::").collect();
                println!("Parsing ipv6");
                Some(IpAddr::V6(Ipv6Addr::new(
                    parse_u16(components[0]),
                    parse_u16(components[1]),
                    parse_u16(components[2]),
                    parse_u16(components[3]),
                    parse_u16(components[4]),
                    parse_u16(components[5]),
                    parse_u16(components[6]),
                    parse_u16(components[7]),
                )))
            }
            _ => None,
        },
    }
}

fn parse_u8(input: &str) -> u8 {
    input.trim().parse::<u8>().unwrap()
}

fn parse_u16(input: &str) -> u16 {
    println!("{}", &input);
    let i = input.as_bytes().to_owned();
    println!("{}", &i[0]);
    println!("{}", &i[1]);

    ((i[0] as u16) << 8) | i[1] as u16
}
