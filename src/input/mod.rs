use std::collections::HashMap;

pub fn get_string_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

pub fn get_masked_input(prompt: &str) -> String {
    rpassword::read_password_from_tty(Some(prompt)).unwrap()
}

#[allow(dead_code)]
pub fn get_input<T>() -> T
where
    T: From<usize>,
{
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to parse input");
    let r = buffer.clone().trim().parse::<usize>().is_ok();
    match r {
        true => T::from(buffer.trim().parse::<usize>().unwrap()),
        _ => T::from(usize::max_value()),
    }
}

pub fn get_option(options: HashMap<u8, &str>) -> u8 {
    let mut i: u8 = 255;
    while !options.contains_key(&i) {
        println!("What type of log are you using?");
        for (key, val) in &options {
            println!("{}) {}", key, val);
        }
        i = get_string_input().trim().parse::<u8>().unwrap();
    }
    i
}
