use super::Log;
use std::collections::HashSet;
use std::net::IpAddr;

pub struct Authentication {}

impl Authentication {
    pub fn unique_ips(logs: Vec<Log>) -> Option<Vec<IpAddr>> {
        let mut unique: HashSet<IpAddr> = HashSet::new();
        for log in logs {
            match log.ip {
                Some(ip) => {
                    if !unique.contains(&ip) {
                        println!("Found unique IP address: {:?}", ip);
                    }
                    unique.insert(ip);
                }
                _ => panic!("Couldn't find an IP address in: {:?}", log.ip),
            }
        }

        Some(unique.into_iter().collect())
    }
}
