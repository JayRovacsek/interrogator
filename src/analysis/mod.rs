mod authentication;

use crate::ingestion::geolocation::Geolocation;
use crate::ingestion::log::Log;
use authentication::Authentication;
use rayon::prelude::*;
use std::net::IpAddr;

pub struct Analysis {
    logs: Vec<Log>,
    geolocations: Option<Vec<Geolocation>>,
    anomalies: Option<Vec<Anomaly>>,
}

#[derive(Clone)]
struct Anomaly {
    log: Log,
    reason: Reason,
}

#[derive(Clone)]
enum Reason {
    Velocity,
    FailedLogins,
    Extension(String),
}

impl Analysis {
    pub fn new(logs: Vec<Log>, geolocations: Option<Vec<Geolocation>>) -> Analysis {
        Analysis {
            logs: logs,
            geolocations: geolocations,
            anomalies: None,
        }
    }

    fn add_anomaly(mut self, log: Log, reason: Reason) {
        let anomoly = Anomaly {
            log: log,
            reason: reason,
        };

        match self.anomalies {
            Some(a) => {
                let mut new_vec = a.clone();
                new_vec.append(&mut vec![anomoly]);
                self.anomalies = Some(new_vec);
            }
            _ => {
                self.anomalies = Some(vec![anomoly]);
            }
        }
    }

    pub fn unique_user_ids(&self) -> Option<Vec<String>> {
        match self.logs.len() {
            n if n > 0 => Authentication::unique_logins(&self.logs),
            _ => unreachable!(
                "Either no logs were passed or something went severly wrong!\nLog count: {}",
                self.logs.len()
            ),
        }
    }

    pub fn get_unqiue_ips(&self) -> Option<Vec<IpAddr>> {
        match self.logs.len() {
            n if n > 0 => Authentication::unique_ips(self.logs.clone()),
            _ => unreachable!(
                "Either no logs were passed or something went severly wrong!\nLog count: {}",
                self.logs.len()
            ),
        }
    }

    pub fn check_velocity(&self) -> Option<Vec<Log>> {
        match self.logs.len() {
            n if n > 0 => None,
            _ => unreachable!(
                "Either no logs were passed or something went severly wrong!\nLog count: {}",
                self.logs.len()
            ),
        }
    }

    pub fn check_common_bots(&self) -> Option<Vec<Log>> {
        match self.logs.len() {
            n if n > 0 => filter_bots(self.logs.clone()),
            // Some(
            // self.logs
            //     .clone()
            //     .into_iter()
            //     .filter(|x| {
            //         match &x.user_agent {
            //             Some(agent) => agent,
            //             None => "",
            //         }
            //         .contains("bot")
            //     })
            //     .collect::<Vec<Log>>(),
            // ),
            _ => unreachable!(
                "Either no logs were passed or something went seriously wrong!\nLog count: {}",
                self.logs.len()
            ),
        }
    }

    pub fn check_auth(&self) -> Option<Vec<Log>> {
        match self.logs.len() {
            n if n > 0 => Some(
                self.logs
                    .clone()
                    .into_iter()
                    .filter(|x| {
                        match &x.user_agent {
                            Some(agent) => agent,
                            None => "",
                        }
                        .contains("bot")
                    })
                    .collect::<Vec<Log>>(),
            ),
            // n if n > 0 => {
            //     // let users = self.logs.into_iter().unique_by(|x| &x.user_id);
            //     // let mut logs = self.logs.clone();
            //     let mut uniques = HashSet::new();
            //     for l in self.logs.clone().iter_mut() {
            //         match &l.user_id {
            //             Some(id) => {
            //                 if uniques.contains(id) {
            //                     uniques.insert(id.clone());
            //                 }
            //             }
            //             _ => panic!("No user id!"),
            //         }
            //     }
            //     // println!("Unique users: {:?}", logs.len());
            //     // logs.retain(|e| uniques.insert(e.clone()));
            //     // println!("Unique users: {:?}", logs.len());
            //     None
            // }
            _ => unreachable!(
                "Either no logs were passed or something went seriously wrong!\nLog count: {}",
                self.logs.len()
            ),
        }
    }

    fn correlate_user(&self, user: String) -> Option<Vec<Log>> {
        Some(
        self.logs
        .clone()
        .into_par_iter()
        .filter(|x| match &x.user_id {
            Some(u) => String::from(u),
            None => String::from("")
        } == user).collect())
    }
}

fn filter_bots(logs: Vec<Log>) -> Option<Vec<Log>> {
    let re = regex::Regex::new(r#"[bot|BOT]"#).unwrap();
    Some(
        logs.into_iter()
            .filter(|x| {
                match &x.user_agent {
                    Some(agent) => match re.captures(agent) {
                        Some(capture) => agent,
                        _ => "",
                    },
                    None => "",
                }
                .contains("bot")
            })
            .collect::<Vec<Log>>(),
    )
}
