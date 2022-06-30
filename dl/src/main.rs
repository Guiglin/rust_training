use std::env;
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Start Download Manager");

    let tab: Vec<_> = env::args().collect();
    assert_ne!(tab.len(), 1);
    let address = &tab[1];
    let response = reqwest::get(address).await?;

    let path = Path::new("./downloaded_file");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let content =  response.bytes().await?;
    file.write_all(&content)?;
    Ok(())
}
