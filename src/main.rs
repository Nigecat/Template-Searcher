use std::{io, fs};
use console::style;
use std::path::Path;
use std::borrow::Cow;
use serde_json::Value;
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
use indicatif::{ProgressStyle, ProgressBar};

// The subreddits to pull data from
const SUBREDDITS: &'static [&'static str] = &["memes", "dankmemes"];

// How many posts to fetch from each subreddit
const LIMIT: &'static usize = &100;



fn basename<'a>(path: &'a String, sep: char) -> Cow<'a, str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}

fn download(url: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut resp = reqwest::blocking::get(url)?;
    let mut out = fs::File::create(Path::new("temp").join(basename(&url, '/').to_string()))?;
    io::copy(&mut resp, &mut out)?;
    Ok(())
}


fn main() {
    println!("{} Requesting posts...", style("[1/2]").bold().dim());

    // A vector to hold urls to the posts we fetch
    let mut posts: Vec<String> = Vec::new();
    
    let pb = ProgressBar::new(SUBREDDITS.len() as u64);
    for subreddit in SUBREDDITS {
        let url = format!("https://www.reddit.com/r/{}/hot.json?limit={}", subreddit, LIMIT); 
        let resp = reqwest::blocking::get(&url).unwrap().json::<Value>().unwrap();

        // Loop over each post and add the image url to our vector of posts
        for i in 0..resp["data"]["children"].as_array().unwrap().len() {
            let val = resp["data"]["children"][i]["data"]["url"].to_string();
            if val != Value::Null {
                posts.push(val);
            }
        }

        pb.inc(1);
    }
    pb.finish_and_clear();



    println!("{} Downloading posts...", style("[2/2]").bold().dim());

    // Delete out temp dir (and ignore any errors since then it already doesn't exist)
    let _ = fs::remove_dir_all("temp");
    // Re-create it so we know it is empty
    fs::create_dir("temp").unwrap();
  
    let pb = ProgressBar::new(posts.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
        .progress_chars("#>-"));

    // Loop over each post and download it
    let i = Arc::new(Mutex::new(0));
    posts.par_iter().for_each(|_post| {
        // For some reason there are quotes around the image url so we have to remove the first and last characters
        let mut post = _post.chars().next().map(|c| &_post[c.len_utf8()..]).unwrap().to_string();
        post.truncate(post.len() - 1);
        let _ = download(&post);
        *i.lock().unwrap() += 1;
        pb.set_position(*i.lock().unwrap());
    });
    pb.finish_and_clear();
}