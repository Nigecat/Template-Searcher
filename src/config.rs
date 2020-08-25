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
    if key == "path" {
        return cfg.path;
    } else if key == "shortcut" {
        return cfg.shortcut;
    } else {
        panic!("unknown field");
    }
}

pub fn set(key: &str, val: String) -> () {
    let mut cfg: Config = confy::load(crate::NAME).unwrap();
    if key == "path" {
        cfg.path = val;
    } else if key == "shortcut" {
        cfg.shortcut = val;
    } else {
        panic!("unknown field");
    }
    confy::store(crate::NAME, cfg).unwrap();
}
