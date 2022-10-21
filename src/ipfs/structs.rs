use serde_yaml;
use serde_yaml::Sequence;
use serde::Deserialize;
use std::collections::HashMap;


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct SubgraphManifest {
    pub dataSources: Vec<DataSource>,
    pub description: String,
    pub repository: String,
    pub specVersion: String,
    pub schema: SchemaAddress
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct SchemaAddress {
    pub file: HashMap<String, String>
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct DataSource {
    pub kind: String,
    pub mapping: Mapping,
    pub name: String,
    pub network: String,
    pub source: Source
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Mapping {
    pub abis: Sequence,
    pub apiVersion: String,
    pub entities: Sequence,
    pub eventHandlers: Sequence,
    pub file: HashMap<String, String>,
    pub kind: String,
    pub language: String
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, PartialEq)]
pub struct Source {
    pub abi: String,
    pub address: String,
    pub startBlock: u32
}

