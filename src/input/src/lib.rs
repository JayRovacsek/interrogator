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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
