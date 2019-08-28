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

    let geo: Vec<Geolocation> = match &user_options.geolocation {
        Some(geo) => Geolocation::from_csv_file(String::from(geo), user_options.clone()),
        _ => Geolocation::from_csv_file(
            String::from("ip2location/ip2location.csv"),
            user_options.clone(),
        ),
    };

    // let geo = Geolocation::from_csv_file(
    //     String::from("ip2location/ip2location.csv"),
    //     user_options.clone(),
    // );

    let logs = thing.ingest_file(target_file);

    let a = Analysis::new(&logs, Some(&geo));
    a.check_auth();
    let bots_logs = a.check_common_bots();

    match bots_logs {
        Some(logs) => println!("Found {} logs related to bots", logs.len()),
        None => println!("No bots found?"),
    }

    if user_options.verbose {
        println!("Logs parsed: {}", logs.len());
        println!("Geolocations parsed: {}", geo.len());
    }

    Ok(())
}
