use anyhow::{anyhow, Result};
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// MAC address to lookup
    mac_address: String,
}

// Did not fully implement deserializing output because you're only interested in the vendor name.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddressInfo {
    vendor_details: VendorDetails,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct AddressError {
    error: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VendorDetails {
    company_name: String,
}

async fn get_company(api_key: &str, mac_address: &str) -> Result<String> {
    let url = format!(
        "https://api.macaddress.io/v1?apiKey={}&output=json&search={}",
        api_key, mac_address
    );
    let response = reqwest::get(url).await?;
    if response.status().is_success() {
        let info: AddressInfo = response.json().await?;
        Ok(info.vendor_details.company_name)
    } else {
        let e: AddressError = response.json().await?;
        Err(anyhow!("{}", e.error))
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mac_address = cli.mac_address;
    let api_key = env::var("MAC_API_KEY").expect("Please set the MAC_API_KEY to your api key");
    match get_company(&api_key, &mac_address).await {
        Ok(company) => println!("Company for MAC address {}: {}", &mac_address, company),
        Err(e) => {
            eprintln!("Error: {:#?}", e);
            std::process::exit(1)
        }
    }
}
