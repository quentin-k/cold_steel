use std::env;
use std::thread;
use std::fs::create_dir_all;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;
use std::path::Path;
use webscreenshotlib as wss;
use reqwest;

// Set the directory for storing screenshots
static OUTPUT_DIR: &str = "output";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create http client for downloading websites

    // parse the arguments
    let args: Vec<String> = env::args().collect();

    // Store the urls into a vector
    let mut target_urls: Vec<String> = Vec::new();
    if let Ok(urls) = read_lines(args[1].clone()) {
        for url in urls {
            if let Ok(curr_url) = url {
                target_urls.push(curr_url);
            }
        }
    }

    // Create directory if it doesn't exist
    match create_dir_all(String::from(OUTPUT_DIR).clone()) {
        Err(_) => println!("Unable to create `{}` directory", OUTPUT_DIR),
        _ => (),
    }

    // iterates through the urls and makes a sceenshot
    for url in target_urls {
        let result = take_screenshots(url.clone());
        match result {
            Err(e) => println!("Error: {}\nFor: {}", e, url),
            _ => (),
        }
    }

    Ok(())
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Take a screenshot of the http and https version of a website
fn take_screenshots(url: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut runtime = tokio::runtime::Runtime::new().unwrap();
    // Do the case for both http and https
    for prefix in &[String::from("http"), String::from("https")] {
        // Create the target
        let target = String::from(prefix) + "://" + &*url;

        // Tell user about execution
        println!("Executing for: {}", target);

        // Create picture variables
        let thread_target = target.clone();
        let pdf_picture = thread::spawn( move || {
            wss::screenshot_tab(&*thread_target, wss::OutputFormat::PDF, 100, true, 1920, 1080, "")
        }).join().unwrap()?;

        let thread_target = target.clone();
        let png_picture = thread::spawn( move || {
            wss::screenshot_tab(&*thread_target, wss::OutputFormat::PNG, 100, true, 1920, 1080, "")
        }).join().unwrap()?;

        // Create the file to store the screenshot
        let mut target_file: String = String::from(OUTPUT_DIR).clone() + "/";
        let split_target = url.split('/');
        for part in split_target {
            target_file = target_file + part + "_";
        }
        target_file = target_file + "_" + prefix;
        let get_src = async{
            match get_page_src(target.clone(), target_file.clone()+".html").await {
                _ => (),
            };
        };
        runtime.block_on(get_src);

        let pdf_target = target_file.clone() + ".pdf";
        let png_target = target_file.clone() + ".png";
        // Write the picture to the file
        match wss::write_screenshot(&*pdf_target, pdf_picture) {
            Err(e) => println!("Error: {}\nWhen writing to: {}", e, pdf_target),
            _ => (),
        }

        match wss::write_screenshot(&*png_target, png_picture) {
            Err(e) => println!("Error: {}\nWhen writing to: {}", e, png_target),
            _ => (),
        }
        println!("Finished processing {}", target);
    }
    Ok(())
}

async fn get_page_src(page: String, dest: String) -> Result<(), Box<dyn std::error::Error>>{
    let page_src  = reqwest::get(&*page)
        .await?
        .text()
        .await?;

    let path = Path::new(&dest);

    let mut file = match File::create(&path) {
        Err(e) => panic!("Unable to create {}\nError: {}", dest, e),
        Ok(file) => file,
    };

    match file.write_all(page_src.as_bytes()) {
        Err(e) => panic!("Unable to write to {}\nError: {}", dest, e),
        Ok(_) => (),
    }

    Ok(())
}
