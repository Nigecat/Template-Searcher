use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    path: String,
    shortcut: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self { Self { path: "".to_string(), shortcut: "".to_string() } }
}

pub fn get(key: &str) -> String {
    let cfg: Config = confy::load(crate::NAME).unwrap();
    return match key {
        "path" => cfg.path,
        "shortcut" => cfg.shortcut,
        _ => panic!("unknown field"),
    }
}

pub fn set(key: &str, val: &str) -> () {
    let mut cfg: Config = confy::load(crate::NAME).unwrap();
    match key {
        "path" => cfg.path = val.to_string(),
        "shortcut" => cfg.shortcut = val.to_string(),
        _ => panic!("unknown field"),
    }
    confy::store(crate::NAME, cfg).unwrap();
}
