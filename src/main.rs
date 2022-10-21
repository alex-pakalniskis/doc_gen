use serde_yaml;
use std::string::String;
use graphql_parser::schema::parse_schema;
use graphql_parser::schema::Definition::TypeDefinition;
use graphql_parser::schema::TypeDefinition::{Scalar, Object, Interface, Union, Enum, InputObject};
mod ipfs;
use ipfs::structs::SubgraphManifest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let manifest_response = reqwest::get("https://ipfs.io/ipfs/QmVsp1bC9rS3rf861cXgyvsqkpdsTXKSnS4729boXZvZyH")
        .await?
        .text()
        .await?;
 
    let deserialized_manifest_data: SubgraphManifest = serde_yaml::from_str(&manifest_response).unwrap();
    
    let schema_hash = deserialized_manifest_data.schema.file.get("/").unwrap();
    
    let subgraph_schema_url = format!("https://ipfs.io{}", schema_hash);
    
    let schema_response = reqwest::get(subgraph_schema_url)
        .await?
        .text()
        .await?;
    
    let ast = parse_schema::<String>(&schema_response)?.to_owned();
    
    for def in ast.definitions {
        match def {
            TypeDefinition(Scalar(s)) => println!("{}: {:?}", s.name, s.description),
            TypeDefinition(Object(o)) => println!("{}: {:?}", o.name, o.description),
            TypeDefinition(Interface(i)) => println!("{}: {:?}", i.name, i.description),
            TypeDefinition(Union(u)) => println!("{}: {:?}", u.name, u.description),
            TypeDefinition(Enum(e)) => println!("{}: {:?}", e.name, e.description),
            TypeDefinition(InputObject(io)) => println!("{}: {:?}", io.name, io.description),
            _ => todo!(),
        }
    }

    
    Ok(())
}