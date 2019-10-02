use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogOptions {
    pub options: HashMap<u8, String>,
}

impl LogOptions {
    pub fn new() -> LogOptions {
        LogOptions {
            options: map! {
                0 as u8 => String::from("Apache"),
                1 as u8 => String::from("Nginx"),
                2 as u8 => String::from("Other")
            },
        }
    }
}
