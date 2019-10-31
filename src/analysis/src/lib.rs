pub mod authentication;

use engine::geolocation::Geolocation;
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
            logs,
            geolocations,
            anomalies: None,
        }
    }

    fn add_anomalies(&mut self, new_anomolies: &mut Vec<Anomaly>) {
        match &self.anomalies {
            Some(a) => {
                let mut new_vec = a.clone();
                new_vec.append(new_anomolies);
                self.anomalies = Some(new_vec);
            }
            _ => {
                self.anomalies = Some(new_anomolies.clone().to_vec());
            }
        }
    }

    fn add_anomaly(mut self, anomaly: Anomaly) {
        match self.anomalies {
            Some(a) => {
                let mut new_vec = a.clone();
                new_vec.append(&mut vec![anomaly]);
                self.anomalies = Some(new_vec);
            }
            _ => {
                self.anomalies = Some(vec![anomaly]);
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

    pub fn get_unqiue_ips(&mut self) -> Option<Vec<IpAddr>> {
        match self.logs.len() {
            n if n > 0 => Authentication::unique_ips(&self.logs),
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

    pub fn check_common_bots(&mut self) {
        let mut anomolies = match self.logs.len() {
            n if n > 0 => filter_bots(&self.logs),
            _ => unreachable!(
                "Either no logs were passed or something went seriously wrong!\nLog count: {}",
                self.logs.len()
            ),
        };
        match anomolies.len() {
            0 => {
                println!("No common bots found.");
            }
            _ => self.add_anomalies(&mut anomolies),
        }
    }

    pub fn check_auth(&self) {
        println!("Check auth stub");
    }

    fn correlate_user(&mut self, user: String) -> Vec<Log> {
        self.logs
        .clone()
        .into_par_iter()
        .filter(|x| match &x.user_id {
            Some(u) => String::from(u),
            None => String::from("")
        } == user).collect()
    }
}

fn filter_bots(logs: &Vec<Log>) -> Vec<Anomaly> {
    let re = regex::Regex::new(r#"(bot)|(BOT)"#).unwrap();

    let anomolous_logs = logs
        .clone()
        .into_par_iter()
        .filter(|x| match &x.user_agent {
            Some(user_agent) => match re.captures(&user_agent) {
                Some(_capture) => true,
                _ => false,
            },
            _ => false,
        })
        .collect::<Vec<Log>>();

    anomolous_logs
        .par_iter()
        .map(|x| Anomaly {
            reason: Reason::Extension(String::from("Bot User")),
            log: x.clone(),
        })
        .collect::<Vec<Anomaly>>()
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
