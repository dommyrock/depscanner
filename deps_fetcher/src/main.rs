use anyhow::{anyhow, Error};
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::time::Duration;
use std::fs::File;
use std::io::{Write, BufWriter};


#[tokio::main]
async fn main() -> Result<(), Error> {
    let options = LaunchOptionsBuilder::default()
        .headless(false)
        .build()
        .unwrap();

    let browser = Browser::new(options).unwrap();

    if let Ok(page) = browser.new_tab() {
        let url = "https://github.com/Qovery/engine/blob/main/Cargo.toml";

        //let random_delay: u64 = rand::thread_rng().gen_range(80..=280) + 650; //ms
        if let Ok(tab) = page.navigate_to(url) {
            std::thread::sleep(Duration::from_millis(1500));
            match tab.find_element("#read-only-cursor-text-area") {
                Ok(elm) => {
                    let _r: Result<Vec<String>, Error> = elm.get_content().and_then(|content| {
                        let mut collecting = false; // Flag to start collecting after [dependencies]
                        let mut deps_lines = Vec::new(); // To store the collected lines

                        // Iterate over lines and collect until [dev-dependencies]
                        for line in content.lines() {
                            if line.trim().starts_with("[dev-dependencies]") {
                                break;
                            }
                            if line.trim().starts_with("[dependencies]") {
                                collecting = true;
                            }
                            if collecting {
                                deps_lines.push(line.to_string());
                            }
                        }

                        if collecting {

                            for l in &deps_lines {
                                println!("{l}");
                            }

                            Ok(deps_lines)
                        } else {
                            Err(anyhow!("No [dependencies] section found"))
                        }
                    });
                }
                Err(e) => {
                    println!("\nError finding element on {}\nError: {}\n", &url, e);
                }
            }
        } else {
            eprint!("Navigation to {url} failed");
        }
    }
    Ok(())
}