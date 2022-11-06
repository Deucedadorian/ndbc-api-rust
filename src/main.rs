use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use mongodb::bson::{doc, Document};
use std::env;
use std::error::Error;
use tokio;
use chrono::{DateTime, Utc};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let wave_data = get_wave_data().await;

    let wave_data = wave_data.expect("No wave data");

    let wave_data = wave_data.split("\n").collect::<Vec<&str>>();

    for station in 0..wave_data.len() {
       if wave_data[station].starts_with("#") == false {
            let v: Vec<&str> = wave_data[station].split_whitespace().collect::<Vec<&str>>();
            println!("{:?}", v);
        }
    }

    //database:
    //
    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URL").expect("You must set the MONGODB_URL environment var!");

    // A Client is needed to connect to MongoDB:
    // An extra line of code to work around a DNS issue on Windows:
    let options =
        ClientOptions::parse_with_resolver_config(&client_uri, ResolverConfig::cloudflare())
        .await?;
    let client = Client::with_options(options)?;

    // Get a handle to a database.
    let db = client.database("swell-ping");

    // List the names of the collections in that database.
    for collection_name in db.list_collection_names(None).await? {
        println!("{}", collection_name);
    }
    
    let collection = db.collection::<Document>("wave-data");

//    println!("the {:#?} collection has been selected.", collection);

    Ok(())
}

async fn get_wave_data() -> Result<String, Box<dyn std::error::Error>> {

    let utc: DateTime<Utc> = Utc::now();
    let hour = utc.format("%H").to_string();
    let url: String = "https://www.ndbc.noaa.gov/data/hourly2/hour_".to_owned() + &hour + ".spec";

    let client = reqwest::Client::new();
    let body = client
        .get(url)
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
