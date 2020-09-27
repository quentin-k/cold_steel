pub mod cold_steel {
    use webscreenshotlib as wss;
    use std::thread;
    pub fn default(url: String) -> Result<(), Box<dyn std::error::Error>> {
        let mut runtime = tokio::runtime::Runtime::new().unwrap();
        // Do the case for both http and https
        for prefix in &[String::from("http"), String::from("https")] {
            // Create the target
            let target = String::from(prefix) + "://" + &*url;
            // Tell user about execution
            println!("Executing for: {}", target);
            // Create picture variables
            let thread_target = target.clone();
            let pdf_picture = thread::spawn(move || {
                wss::screenshot_tab(
                    &*thread_target,
                    wss::OutputFormat::PDF,
                    100,
                    true,
                    1920,
                    1080,
                    "",
                )
            })
            .join()
            .unwrap()?;
            let thread_target = target.clone();
            let png_picture = thread::spawn(move || {
                wss::screenshot_tab(
                    &*thread_target,
                    wss::OutputFormat::PNG,
                    100,
                    true,
                    1920,
                    1080,
                    "",
                )
            })
            .join()
            .unwrap()?;
            // Create the file to store the screenshot
            let mut target_file: String = String::from(crate::config::OUTPUT_DIR).clone() + "/";
            let split_target = url.split('/');
            for part in split_target {
                target_file = target_file + part + "_";
            }
            target_file = target_file + "_" + prefix;
            let get_src = async {
                match crate::network::network::get_page_src(target.clone(), target_file.clone() + ".html")
                    .await
                {
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
}
