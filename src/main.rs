use std::time::Duration;
use futures::stream::{StreamExt};
use std::io::Write;
use regex::Regex;

mod model;
mod client;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Rokrs is starting scanning local network for roku devices!");
    let mut devices = scan().await?;
    let mut input = String::new();
    while devices.len() == 0 {
        println!("No devices detected!");
        print!("Would you like to scan again? (y/n)");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        if input.split(" ").next().unwrap().to_lowercase() == "n" {
            return Ok(());
        }
        devices = scan().await?;
    }
    let mut idx: usize;
    loop {
        let mut input = String::new();
        print!("Please input the number of the device you want to control: ");
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut input)?;
        idx = input.trim().parse::<usize>().expect("Expected a integer representing device number!") - 1;
        if idx < devices.len() {
            break;
        }
        input.clear();
        println!("\nInvalid device index passed try again!\n")
    }
    let mut device = devices.get_mut(idx).unwrap();
    println!("Selected device: {}", device.info.user_device_name);
    println!("Starting rokrs control console!");
    let regex = Regex::new(r#"(?m)("[^"]+"|[^\s"]+)"#).unwrap();
    loop {
        print!("({}) ➜ ", device.info.user_device_name);
        std::io::stdout().flush()?;
        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        let mut args = regex.find_iter(input.trim());
        match args.next().unwrap().as_str() {
            "keypress" => {
                device.keypress(args.next().unwrap().as_str()).await.unwrap();
            },
            "launch" => {
                device.launch_app(args.next().unwrap().as_str()).await.unwrap();
            },
            "install" => {
                device.install_app(args.next().unwrap().as_str()).await.unwrap();
            },
            "rescan" => {
                devices = scan().await?;
                loop {
                    let mut input = String::new();
                    print!("Please input the number of the device you want to control: ");
                    std::io::stdout().flush()?;
                    std::io::stdin().read_line(&mut input)?;
                    idx = input.trim().parse::<usize>().expect("Expected a integer representing device number!") - 1;
                    if idx < devices.len() {
                        break;
                    }
                    input.clear();
                    println!("\nInvalid device index passed try again!\n")
                }
                device = devices.get_mut(idx).unwrap();
            },
            "tv-channels" => {
                let channels = device.tv_channels().await.unwrap();
                println!("{:?}", channels);
            },
            "active-channel" => {
                let channel = device.active_channel().await.unwrap();
                println!("{:?}", channel);
            },
            "exit" => {
                println!("Exiting.");
                std::process::exit(0);
            },
            _ => {
                println!("Unknown command.");
            },
        }
    }
}

async fn scan() -> std::io::Result<Vec<client::Device>> {
    let mut responses = ssdp_client::search(&ssdp_client::SearchTarget::Custom("roku".to_string(), "ecp".to_string()), Duration::from_secs(3), 2).await.unwrap();
    let mut devices = vec![];
    while let Some(response) = responses.next().await {
        let response = response.unwrap();
        let device = fetch(response.location()).await.unwrap();
        println!("{}. {}", devices.len()+1, device.model_name);
        println!("Name: {}", device.user_device_name);
        println!("Location: {}", device.user_device_location.clone().unwrap_or("Unknown".to_string()));
        println!("Vendor: {}\n", device.vendor_name);
        devices.push(client::new(response.location().to_string(), device));
    }
    Ok(devices)
}

use serde_xml_rs::{from_str};
//fetches info on a given roku device
pub async fn fetch<'a>(device_url: &'a str) -> Result<crate::model::DeviceInfo, reqwest::Error> {
    let resp = reqwest::get(format!("{}query/device-info", device_url))
        .await?
        .text()
        .await?;
    Ok(from_str(&*resp).unwrap())
}