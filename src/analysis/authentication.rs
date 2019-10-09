use super::Log;
use std::collections::HashSet;
use std::net::IpAddr;

pub struct Authentication {}

impl Authentication {
    pub fn unique_ips(logs: &Vec<Log>) -> Option<Vec<IpAddr>> {
        let mut unique: HashSet<IpAddr> = HashSet::new();
        // Some(
        //     logs.iter()
        // .map(|x| {
        //                 match x.ip {
        //         Some(ip) => {
        //             if !unique.contains(&ip) {
        //                 println!("Found unique IP address: {:?}", ip);
        //             }
        //             unique.insert(*ip);
        //         }
        //         _ => panic!("Couldn't find an IP address in: {:?}", x.ip),
        //     }
        // }))
        for log in logs {
            match log.ip {
                Some(ip) => {
                    // if !unique.contains(&ip) {
                    //     println!("Found unique IP address: {:?}", ip);
                    // }
                    unique.insert(ip);
                }
                _ => panic!("Couldn't find an IP address in: {:?}", log.ip),
            }
        }

        Some(unique.into_iter().collect())
    }

    pub fn unique_logins(logs: &Vec<Log>) -> Option<Vec<String>> {
        let mut unique: HashSet<String> = HashSet::new();
        for log in logs {
            match &log.user_id {
                Some(id) => {
                    if !unique.contains(id) {
                        println!("Found user id: {:?}", id);
                    }
                    unique.insert(String::from(id));
                }
                _ => println!("Not a unique user: {:?}", log.user_id),
            }
        }
        Some(unique.into_iter().collect())
    }
}
