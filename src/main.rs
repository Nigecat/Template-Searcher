use console::style;
use serde_json::Value;
use indicatif::ProgressBar;

// The subreddits to pull data from
const SUBREDDITS: &'static [&'static str] = &["memes", "dankmemes"];

// How many posts to fetch from each subreddit
const LIMIT: &'static usize = &100;

fn main() {
    println!("{} Requesting posts...", style("[1/1]").bold().dim());

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

    println!("{:#?}", posts);
}