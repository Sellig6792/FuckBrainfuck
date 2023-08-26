use anyhow::Context;
use colored::Colorize;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde_json::Value;
use std::io::Write;
use std::ops::Not;
use std::{env, fs};

use crate::version::Version;
// This macro returns the github api url for the given repository
macro_rules! get_api_url {
    ($repo_url:expr) => {
        format!(
            "https://api.github.com/repos/{}",
            $repo_url.replace("https://github.com/", "")
        )
    };
}

fn get_api_json(endpoint: &str) -> Value {
    let url = get_api_url!(env!("CARGO_PKG_REPOSITORY"));

    let headers = {
        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static(env!("CARGO_PKG_NAME")));
        headers
    };
    let client = reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let response = match client.get(format!("{}/{}", url, endpoint)).send() {
        Ok(response) => response,
        Err(error) => panic!("Could not get latest release: {}", error),
    };

    let json: Value = match response.text() {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(json) => json,
            Err(_) => panic!("Could not parse the response from the github api {}", text),
        },
        Err(_) => panic!("Could not read the response from the github api"),
    };

    json
}

pub fn update(force_update: bool) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(not(fuckbf_updatable))]
    {
        println!(
            "No precompiled binary for this target, please compile from source.\
                For more information,see {}#2-building-from-source",
            env!("CARGO_PKG_HOMEPAGE")
        );
        std::process::exit(0);
    }

    let binding_api_latest_release = get_api_json("releases/latest");

    let version = Version::parse(
        binding_api_latest_release
            .get("name")
            .unwrap()
            .as_str()
            .unwrap(),
    );
    let current_version = Version::parse(env!("CARGO_PKG_VERSION"));
    if current_version >= version && force_update.not() {
        println!("     {} up-to-date", "Already".green().bold());
        std::process::exit(0);
    }

    let binding_current_exe = env::current_exe().unwrap();
    let installation_dir = binding_current_exe.parent().unwrap();
    let assets = binding_api_latest_release
        .get("assets")
        .unwrap()
        .as_array()
        .unwrap();
    let download_url: String = assets
        .iter()
        .find(|asset| asset.get("name").unwrap().as_str().unwrap() == env!("FUCKBF_BINARY_NAME"))
        .with_context(|| {
            format!(
                "No precompiled binary for this target, please compile from source.\
            For more information,see {}#2-building-from-source",
                env!("CARGO_PKG_HOMEPAGE")
            )
        })?
        .get("browser_download_url")
        .with_context(|| {
            format!(
                "No precompiled binary for this target, please compile from source.\
            For more information,see {}#2-building-from-source",
                env!("CARGO_PKG_HOMEPAGE")
            )
        })?
        .to_string()
        .replace('"', "");

    println!(
        " {} latest version ({})",
        "Downloading".green().bold(),
        download_url
    );

    let binary = reqwest::blocking::get(&download_url)
        .unwrap()
        .bytes()
        .unwrap();

    println!("  {} latest version", "Installing".green().bold());

    let path = installation_dir.join("fuckbf.new");

    // Write the binary to the current directory
    let mut file = fs::File::create(&path).expect("Could not create the new binary");
    file.write_all(&binary)
        .expect("Could not write to the file");

    let current_binary = env::current_exe().unwrap();

    // Move current binary to a .old file
    fs::rename(&current_binary, installation_dir.join("fuckbf.old"))
        .expect("Could not rename the current binary");

    // Move the new binary to the current binary
    fs::rename(&path, &current_binary).expect("Could not rename the new binary");

    println!("     {} latest version", "Updated".green().bold());

    Ok(())
}

// Delete the {}.old file if it exists it was generated by a previous update (the file is in the same directory as the executable)
pub fn delete_old_file() {
    if fs::metadata("fuckbf.old").is_ok() {
        if let Err(err) = fs::remove_file("fuckbf.old") {
            eprintln!(
                "Error while deleting the old version from your system: {:?}",
                err
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_api_url() {
        let repo_url = "https://github.com/user/repo";
        assert_eq!(
            get_api_url!(repo_url),
            "https://api.github.com/repos/user/repo"
        )
    }

    #[test]
    fn test_get_latest_release() {
        // The code checks itself that the status of the request is OK
        get_api_json("releases/latest");
    }
}