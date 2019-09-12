mod analysis;
mod ingestion;
mod input;
mod okta;
mod program_options;

#[macro_use]
extern crate clap;
extern crate chrono;
extern crate lazy_static;
extern crate rayon;
extern crate regex;
extern crate rpassword;
#[macro_use]
extern crate serde_derive;

use analysis::Analysis;
use clap::App;
use ingestion::geolocation::Geolocation;
use ingestion::ingestor::Ingestor;
use program_options::ProgramOptions;
use std::io::Result;

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);

fn main() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let user_options = ProgramOptions::new(&matches);
    let target_file = matches
        .value_of("input")
        .expect("An error occured reading the input file!");

    let log_options = map! {
        0 as u8 => "Apache",
        1 as u8 => "Nginx",
        2 as u8 => "Other"
    };

    let option = input::get_option();

    let mut thing = Ingestor::new(option, log_options, user_options.clone());

    let mut geo: Option<Vec<Geolocation>> = None;

    if matches.is_present("geolocation") {
        geo = match &user_options.geolocation {
            Some(geo) => Some(Geolocation::from_csv_file(
                String::from(geo),
                user_options.clone(),
            )),
            _ => Some(Geolocation::from_csv_file(
                String::from("ip2location/ip2location.csv"),
                user_options.clone(),
            )),
        };
    }

    let logs = thing.ingest_file(target_file);

    let a = match &geo {
        Some(g) => Analysis::new(logs.clone(), Some(g.clone())),
        _ => Analysis::new(logs.clone(), None),
    };

    a.check_auth();
    let bots_logs = a.check_common_bots();
    let unique_ips = a.get_unqiue_ips();
    let unique_logins = a.unique_user_ids();

    match bots_logs {
        Some(l) => println!("Found {} logs related to bots", l.len()),
        None => println!("No bots found?"),
    }

    if user_options.verbose {
        match unique_ips {
            Some(l) => println!("Found {} logs have unique ips", l.len()),
            None => println!("No unique ips found?"),
        }

        match unique_logins {
            Some(l) => println!("Found {} logs have unique user ids", l.len()),
            None => println!("No unique user ids found?"),
        }

        match geo {
            Some(g) => println!("Geolocations parsed: {}", g.len()),
            None => println!("No geolocations parsed"),
        }
        println!("Logs parsed: {}", logs.len());
    }

    Ok(())
}
