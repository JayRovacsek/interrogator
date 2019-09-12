use crate::ProgramOptions;
use rayon::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

#[derive(Debug, Clone)]
pub struct Geolocation {
    start: IpAddr,
    end: IpAddr,
    country_code: String,
    country_name: String,
    region: String,
    city: String,
    latitude: f32,
    longitude: f32,
}

impl Geolocation {
    pub fn from_str(input: &str) -> Geolocation {
        let components: Vec<String> = strip_csv_str(String::from(input));
        Geolocation {
            start: parse_ip_address(components[0].parse::<usize>().unwrap()), // ipv4 max 4294967295
            end: parse_ip_address(components[1].parse::<usize>().unwrap()),
            country_code: components[2].to_owned(),
            country_name: components[3].to_owned(),
            region: components[4].to_owned(),
            city: components[5].to_owned(),
            latitude: components[6].parse::<f32>().unwrap(),
            longitude: components[7].parse::<f32>().unwrap(),
        }
    }

    pub fn from_csv_file(input: String, program_options: ProgramOptions) -> Vec<Geolocation> {
        let mut geolocations: Vec<Geolocation> = Vec::new();

        let file = BufReader::new(File::open(input).unwrap());
        let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

        match program_options.sequential {
            true => lines
                .par_iter()
                .map(|line| Geolocation::from_str(line))
                .collect_into_vec(&mut geolocations),
            false => lines
                .par_iter()
                .map(|line| Geolocation::from_str(line))
                .collect_into_vec(&mut geolocations),
        }
        geolocations
    }
}

impl std::fmt::Display for Geolocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "IP Range Start: {:?}
            IP Range End: {:?}
            Country Code: {:?}
            Country Name: {:?}
            Region: {:?}
            City: {:?}
            Latitude: {:?}
            Longitude: {:?}",
            self.start,
            self.end,
            self.country_code,
            self.country_name,
            self.region,
            self.city,
            self.latitude,
            self.longitude
        )
    }
}

fn strip_csv_str(input: String) -> Vec<String> {
    input
        .split("\",")
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .iter()
        .map(|x| str::replace(x, "\"", ""))
        .collect()
}

fn parse_ip_address(input: usize) -> IpAddr {
    match input {
        0..=4_294_967_295 => parse_ipv4(input),
        _ => parse_ipv6(input),
    }
}

fn parse_ipv4(input: usize) -> IpAddr {
    let w: u8 = ((input / 16_777_216) % 256) as u8;
    let x: u8 = ((input / 65536) % 256) as u8;
    let y: u8 = ((input / 256) % 256) as u8;
    let z: u8 = (input) as u8;
    IpAddr::V4(Ipv4Addr::new(w, x, y, z))
    // Ipv4Addr::new(w, x, y, z)

    // TODO: ADD BELOW CHECKS IN VERBOSE MODE
    // let vals = [w, x, y, z];
    // match vals {
    //     [0, 0, 0, 0] => {
    //         panic!(
    //             "May have been an issue with: {:?}, input was: {:?}",
    //             Ipv4Addr::new(w, x, y, z),
    //             input
    //         );
    //     }
    //     [255, 255, 255, 255] => {
    //         panic!(
    //             "May have been an issue with: {:?}, input was: {:?}",
    //             Ipv4Addr::new(w, x, y, z),
    //             input
    //         );
    //     }
    //     _ => Ipv4Addr::new(w, x, y, z),
    // }
}
// IP Address = w.x.y.z
// To reverse IP number to IP address,
// w = int ( IP Number / 16777216 ) % 256
// x = int ( IP Number / 65536    ) % 256
// y = int ( IP Number / 256      ) % 256
// z = int ( IP Number            ) % 256

fn parse_ipv6(input: usize) -> IpAddr {
    let a: u16 = (input / (65536 ^ 7) % 65536) as u16;
    let b: u16 = (input / (65536 ^ 6) % 65536) as u16;
    let c: u16 = (input / (65536 ^ 5) % 65536) as u16;
    let d: u16 = (input / (65536 ^ 4) % 65536) as u16;
    let e: u16 = (input / (65536 ^ 3) % 65536) as u16;
    let f: u16 = (input / (65536 ^ 2) % 65536) as u16;
    let g: u16 = (input / (65536) % 65536) as u16;
    let h: u16 = (input % 65536) as u16;
    IpAddr::V6(Ipv6Addr::new(a, b, c, d, e, f, g, h))
}
// IP Address = a:b:c:d:e:f:g:h
// To reverse IP number to IP address,
// a = int ( IP Number / (65536^7) ) % 65536
// b = int ( IP Number / (65536^6) ) % 65536
// c = int ( IP Number / (65536^5) ) % 65536
// d = int ( IP Number / (65536^4) ) % 65536
// e = int ( IP Number / (65536^3) ) % 65536
// f = int ( IP Number / (65536^2) ) % 65536
// g = int ( IP Number / 65536 ) % 65536
// h = IP Number % 65536
