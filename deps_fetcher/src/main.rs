use anyhow::Error;
use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let options = LaunchOptionsBuilder::default()
        // .headless(false)
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
                    let _r: Result<(), Error> = elm.get_content().and_then(|content| {
                        let (mut collecting, mut collecting_dev_deps) = (false, false);
                        let mut deps_lines = Vec::new();
                        let mut dev_deps_lines = Vec::new();

                        for line in content.lines() {
                            match line.trim() {
                                s if s.starts_with("[dev-dependencies]") => {
                                    collecting_dev_deps = true;
                                    collecting = false;
                                }
                                s if s.starts_with("[")
                                    && !line.trim().contains("dependencies")
                                    && !line.trim().contains("dev-dependencies") =>
                                {
                                    break;
                                }
                                s if s.starts_with("[dependencies]") => {
                                    collecting = true;
                                    collecting_dev_deps = false;
                                }
                                _ => {}
                            }
                            if collecting {
                                deps_lines.push(line.to_string());
                            } else if collecting_dev_deps {
                                dev_deps_lines.push(line.to_string());
                            }
                        }

                        write_lines_to_file(&deps_lines, "dependencies.txt")?;
                        write_lines_to_file(&dev_deps_lines, "dev_dependencies.txt")?;
                        Ok(())
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

fn write_lines_to_file(lines: &[String], file_name: &str) -> Result<(), std::io::Error> {
    let file = File::create(file_name)?;
    let mut writer = BufWriter::new(file);
    for line in lines {
        writer.write_all(line.as_bytes())?;
        writer.write_all(b"\n")?;
    }
    Ok(())
}
