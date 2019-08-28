use crate::input;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProgramOptions {
    pub input: String,
    pub geolocation: Option<String>,
    pub rate_limit: usize,
    pub api_key: String,
    pub sequential: bool,
    pub output: Option<String>,
    pub verbose: bool,
}

impl ProgramOptions {
    pub fn new(matches: &clap::ArgMatches) -> ProgramOptions {
        let mut user_options: ProgramOptions = ProgramOptions {
            input: matches
                .value_of("input")
                .unwrap()
                .trim()
                .parse::<String>()
                .unwrap(),
            geolocation: None,
            rate_limit: 150 as usize,
            api_key: String::new(),
            sequential: false,
            output: None,
            verbose: false,
        };

        if matches.is_present("geolocation") {
            let geo = matches
                .value_of("geolocation")
                .unwrap();
            user_options.geolocation = Some(String::from(geo));
        }

        if matches.is_present("rate_limit") {
            let rate = matches
                .value_of("rate_limit")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            user_options.rate_limit = match rate {
                1...200 => rate,
                _ => 150,
            };
        };

        if matches.is_present("api_key") {
            user_options.api_key = String::from(matches.value_of("api_key").unwrap_or_default());
        } else {
            user_options.api_key = input::get_masked_input("Please enter API key:");
        }

        if matches.is_present("sequential") {
            user_options.sequential = true;
        };

        if matches.is_present("output") {
            user_options.output = Some(String::from(
                matches.value_of("output").unwrap_or("results.json"),
            ));
        };

        if matches.is_present("verbose") {
            user_options.verbose = true;
        };

        user_options
    }
}

impl std::fmt::Display for ProgramOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Input: {}
            Rate limit: {}
            API key: {}
            Sequential: {}
            Output: {}
            Verbose: {}
            '",
            self.input,
            self.rate_limit,
            self.api_key,
            self.sequential,
            self.output.clone().unwrap_or_default(),
            self.verbose
        )
    }
}