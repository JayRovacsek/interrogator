use crate::log_options::LogOptions;
use std::collections::HashMap;

pub fn get_string_input(prompt: Option<&str>) -> String {
    let mut buffer = String::new();
    match prompt {
        Some(p) => println!("{}", p),
        _ => print!(""),
    };
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn get_masked_input(prompt: &str) -> String {
    match rpassword::read_password_from_tty(Some(prompt)) {
        Ok(result) => {
            if result == String::default() {
                get_masked_input(prompt)
            } else {
                result
            }
        }
        _ => get_masked_input(prompt),
    }
}

pub fn get_option(prompt: Option<&str>, options: Option<LogOptions>) -> u8 {
    let input = match &prompt {
        Some(p) => get_string_input(Some(p)),
        _ => match &options {
            Some(o) => {
                let options_prompt = o
                    .options
                    .iter()
                    .fold(String::from("Please enter log type:"), |s, option| {
                        format!("{}\n{:?}: {:?}", s, option.0, option.1)
                    });
                get_string_input(Some(&options_prompt))
            }
            _ => panic!("No prompt or options were passed to get_option function"),
        },
    };

    match input.trim().parse::<u8>() {
        Ok(val) => val,
        _ => {
            println!("Input didn't match expected range: 0 - 255");
            get_option(prompt, options)
        }
    }
}
