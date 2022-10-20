use serde_yaml;
use serde_yaml::Sequence;
use serde::Deserialize;
use std::collections::HashMap;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct Yml {
    dataSources: Vec<DataSource>
}

#[derive(Debug, Deserialize, PartialEq)]
struct DataSource {
    kind: String,
    mapping: Mapping,
    name: String,
    network: String,
    source: Source
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct Mapping {
    abis: Sequence,
    apiVersion: String,
    entities: Sequence,
    eventHandlers: Sequence,
    file: HashMap<String, String>,
    kind: String,
    language: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
struct Source {
    abi: String,
    address: String,
    startBlock: u32
}



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://ipfs.io/ipfs/Qmbzn47G3NBgHuDyFqXaf646SCRz2CK93VBkUD3AV7nGtk")
        .await?
        .text()
        .await?;
    
    let deserialized_manifest_data: Yml = serde_yaml::from_str(&resp).unwrap();

    println!("{:#?}", deserialized_manifest_data);
    Ok(())
}

