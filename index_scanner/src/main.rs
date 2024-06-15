
use reqwest::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Dependency {
    name: String,
    vers: String,
    deps: Vec<Dep>,
    cksum: String,
    features: HashMap<String, Vec<String>>,
    yanked: bool,
    rust_version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Dep {
    name: String,
    req: String,
    features: Vec<String>,
    optional: bool,
    default_features: bool,
    target: Option<String>,
    kind: Option<String>,
    crates_url: Option<String>,
}

/*
docs
https://doc.rust-lang.org/cargo/reference/registry-index.html#index-files
*/

#[tokio::main]
async fn main() -> Result<(), Error> {
    //https://index.crates.io/<first_two>/<next_two>/<crate_name>
    let url = "https://index.crates.io/ba/se/base64"; //or https://index.crates.io/se/rd/serde_json

    let response = reqwest::get(url).await?;
    let content = response.text().await?;

    let mut result = Vec::new();

    // Split the content if there are multiple JSON objects (assuming newlines separate them)
    for json_object in content.split("\n").filter(|s| !s.trim().is_empty()) {
        match serde_json::from_str::<Dependency>(&json_object) {
            Ok(mut dependency) => {
                // println!("{:#?}", dependency);
                dependency
                    .deps
                    .iter_mut()
                    .for_each(|d| d.crates_url = Some(format!("https:/crates.io/crates/{}", d.name)));

                result.push(dependency);
            }
            Err(e) => {
                eprintln!("Failed to deserialize JSON: {}", e);
            }
        }
    }

    //for now i take only the latest version
    let latest = result.last();
    println!("{:#?}", latest.unwrap());

    Ok(())
}

//1 get the tomld divs
//2 get list
//3 query crates.io
//4 construct version dif list foreach one
//5 render some static html reort with difs anad links to changelogs
