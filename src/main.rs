mod cold_steel;
mod config;
mod files;
mod network;

use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create http client for downloading websites

    // Parse the arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("This program takes at least 2 arguments!");
        return Ok(());
    }

    // Store the urls into a vector
    let mut target_urls: Vec<String> = Vec::new();

    if let Ok(urls) = files::files::get_lines_from(args[1].clone()) {
        for url in urls {
            if let Ok(curr_url) = url {
                target_urls.push(curr_url);
            }
        }
    }

    // Create directory if it doesn't exist
    files::files::create_dir(config::OUTPUT_DIR.to_string());

    // Iterates through the urls and makes a screenshot
    for url in target_urls {
        let result = cold_steel::cold_steel::default(url.clone());
        match result {
            Err(e) => println!("Error: {}\nFor: {}", e, url),
            _ => (),
        }
    }

    Ok(())
}
