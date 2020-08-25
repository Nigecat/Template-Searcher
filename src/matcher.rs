use glob::glob;
use regex::Regex;
use std::path::Path;
use std::ffi::OsStr;
use lazy_static::lazy_static;

pub fn get_match(mut search: String, path: String) -> String {
    lazy_static! {
        static ref MATCH_LEVEL_EXTRACTOR: Regex = Regex::new(r#"\d+$"#).unwrap();
    }

    let base = Path::new(&path);

    // Read all the files (recursively) in the target directory
    let files: Vec<_> = glob(base.join("**").join("*").to_str().unwrap()).unwrap().map(|res| res.unwrap().into_os_string().into_string().unwrap()).collect();
    let file_names = files.iter().map(|file| OsStr::to_str(Path::new(&file).file_name().unwrap()).unwrap().to_string()).collect::<Vec<_>>();

    // Ensure our files vector and our file name vectors are the same length
    assert_eq!(files.len(), file_names.len());

    // Get the last character as the match level, default to 1 (the search will get the nth closed match)
    let mut match_level = 1;
    let mut match_temp;
    
    // Override the default match level if there are digits at the end
    if MATCH_LEVEL_EXTRACTOR.is_match(&search) {
        match_level = MATCH_LEVEL_EXTRACTOR.find(&search).unwrap().start();
        search = MATCH_LEVEL_EXTRACTOR.replace_all(&search, "").to_string();
    }

    // Check if a file starts with the search string
    match_temp = match_level;
    for i in 0..file_names.len() {
        if file_names[i].starts_with(&search) {
            match_temp -= 1;
            if match_temp == 0 {
                return files[i].clone();
            }
        }
    }

    // Check if a file contains the search string
    match_temp = match_level;
    for i in 0..file_names.len() {
        if file_names[i].contains(&search) {
            match_temp -= 1;
            if match_temp == 0 {
                return files[i].clone();
            }
        }
    }


    // If a file contains all the words in the search str
    match_temp = match_level;
    for i in 0..file_names.len() {
        let mut valid = true;
        for word in search.split(" ") {
            if !file_names[i].contains(word) {
                valid = false;
            }
        }

        if valid {
            match_temp -= 1;
            if match_temp == 0 {
                return files[i].clone();
            }
        }
    }

    return "".to_string();
}