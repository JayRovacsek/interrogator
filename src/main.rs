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

    let option = input::get_option(log_options.clone());

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
        Some(geo) => Analysis::new(&logs, Some(&geo)),
        _ => Analysis::new(&logs, None),
    };

    // let geo = Geolocation::from_csv_file(
    //     String::from("ip2location/ip2location.csv"),
    //     user_options.clone(),
    // );

    // let logs = thing.ingest_file(target_file);

    // let a = Analysis::new(&logs, Some(&geo));
    a.check_auth();
    let bots_logs = a.check_common_bots();
    let unique_ips = a.get_unqiue_ips();

    match bots_logs {
        Some(logs) => println!("Found {} logs related to bots", logs.len()),
        None => println!("No bots found?"),
    }

    match unique_ips {
        Some(logs) => println!("Found {} logs have unique ips", logs.len()),
        None => println!("No unique ips found?"),
    }

    if user_options.verbose {
        match geo {
            Some(geo) => println!("Geolocations parsed: {}", geo.len()),
            None => println!("No geolocations parsed"),
        }
        println!("Logs parsed: {}", logs.len());
    }

    Ok(())
}
