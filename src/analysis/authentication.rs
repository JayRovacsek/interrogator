use super::Log;
use std::net::IpAddr;

pub struct Authentication {}

impl Authentication {
    pub fn unique_ips(logs: Vec<Log>) -> Option<Vec<IpAddr>> {
        let mut unique: Vec<IpAddr> = Vec::new();
        logs.into_iter().map(|x| match x.ip {
            Some(ip) => {
                if !unique.contains(&ip) {
                    unique.push(ip);
                };
            }
            _ => panic!("Couldn't find an IP address in:\n{:?}", x),
        });
        Some(unique)
    }
}
