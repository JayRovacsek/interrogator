
pub fn get_string_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn get_masked_input(prompt: &str) -> String {
    match rpassword::read_password_from_tty(Some(prompt)) {
        Ok(result) => {
            if result == String::default() { get_masked_input(prompt) } else { result }
        },
        _ => get_masked_input(prompt),
    }
}

pub fn get_option() -> u8 {
    let input = get_string_input();
    match input.trim().parse::<u8>() {
        Ok(val) => val,
        _ => {
            println!("Input didn't match expected range: 0 - 255");
            get_option()
        }
    }
}
