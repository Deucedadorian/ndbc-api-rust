use mongodb::{Client, options::{ClientOptions, ResolverConfig}};
use std::env;
use std::error::Error;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let wave_data = get_wave_data().await;

    let wave_data = wave_data.expect("No wave data");

    let wave_data = wave_data.split("\n").collect::<Vec<&str>>();

//    println!("{:#?}", wave_data);

    let mut this_is_vec = Vec::new();
 
    for (i, station) in 0..wave_data.len() {
        println!("{}", i);
        println!("{}", station);
        let s_data : u16 = station;
        if !s_data.starts_with("#") {
            let v: Vec<&str> = wave_data[station].split_whitespace().collect::<Vec<&str>>();
            this_is_vec.push(v);
        }
    }

    println!("{:#?}", this_is_vec);

    // Load the MongoDB connection string from an environment variable:
    let client_uri =
        env::var("MONGODB_URI").expect("You must set the MONGODB_URI environment var!");

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

    //database:

    //let swell_data = client.database("swell-ping").collection("wave-data");

    Ok(())
}

async fn get_wave_data() -> Result<String, Box<dyn std::error::Error>> {
    // use reqwest::header::{CONTENT_TYPE};

    let client = reqwest::Client::new();
    let body = client
        .get("https://www.ndbc.noaa.gov/data/hourly2/hour_00.spec")
        // .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .text()
        .await?;

    Ok(body)
}
