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
        let mut program_options: ProgramOptions = ProgramOptions {
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
            let geo = matches.value_of("geolocation").unwrap();
            program_options.geolocation = Some(String::from(geo));
        }

        if matches.is_present("rate_limit") {
            let rate = matches
                .value_of("rate_limit")
                .unwrap()
                .trim()
                .parse::<usize>()
                .unwrap();

            program_options.rate_limit = match rate {
                1..=200 => rate,
                _ => 150,
            };
        };

        if matches.is_present("api_key") {
            program_options.api_key = String::from(matches.value_of("api_key").unwrap_or_default());
        } else {
            program_options.api_key = input::get_masked_input("Please enter API key:");
        }

        if matches.is_present("sequential") {
            program_options.sequential = true;
        };

        if matches.is_present("output") {
            program_options.output = Some(String::from(
                matches.value_of("output").unwrap_or("results.json"),
            ));
        };

        if matches.is_present("verbose") {
            program_options.verbose = true;
        };

        if program_options.verbose {
            println!("Loading program settings, they appear to be:");
            println!("{}", program_options);
        }

        program_options
    }
}

fn mask_output(input: &str) -> String {
    let mut result = String::from(input);
    let range = match input.len() < 5 {
        true => input.len(),
        _ => input.len() - 4,
    };
    let mask = (0..range).fold(String::new(), |s, _u| s + "*");

    result.replace_range(..range, &mask);
    result
}

impl std::fmt::Display for ProgramOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let geo = match &self.geolocation {
            Some(geo) => geo,
            _ => "No geolocation file in options",
        };

        let output = match &self.output {
            Some(output) => output,
            _ => "No output location in options",
        };

        write!(
            f,
            "#######################\n
Input:\t\t\t{}
Rate limit:\t\t{}
Geolocation:\t\t{}
API key:\t\t{}
Sequential:\t\t{}
Output:\t\t\t{}
Verbose:\t\t{}\n
#######################",
            self.input,
            self.rate_limit,
            geo,
            mask_output(&self.api_key),
            self.sequential,
            output,
            self.verbose
        )
    }
}
