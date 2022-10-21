use serde_yaml;
use std::string::String;
use graphql_parser::schema::parse_schema;
use graphql_parser::schema::Field;
use graphql_parser::schema::EnumValue;
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

    let mut names: Vec<String> = Vec::new();
    let mut descriptions: Vec<Option<String>> = Vec::new();
    let mut fields: Vec<Vec<Field<String>>> = Vec::new();
    let mut enum_values: Vec<Vec<EnumValue<String>>> = Vec::new();


    for def in ast.definitions {

        let mut tmp_fields: Vec<Field<String>> = Vec::new();
        let mut tmp_enum_values: Vec<EnumValue<String>> = Vec::new();

        match def {
            TypeDefinition(Scalar(s)) => println!("{}: {:?}", s.name, s.description),
            TypeDefinition(Object(o)) => {
                names.push(o.name);
                descriptions.push(o.description);

                for field in o.fields {
                    tmp_fields.push(field)
                }
                fields.push(tmp_fields);
            },
            TypeDefinition(Interface(i)) => println!("{}: {:?}", i.name, i.description),
            TypeDefinition(Union(u)) => println!("{}: {:?}", u.name, u.description),
            TypeDefinition(Enum(e)) => {
                names.push(e.name);
                descriptions.push(e.description);

                for value in e.values {
                    tmp_enum_values.push(value)
                }
                enum_values.push(tmp_enum_values);
            },
            TypeDefinition(InputObject(io)) => println!("{}: {:?}", io.name, io.description),
            _ => todo!(),
        }
    }

    println!("{:?}\n", names);
    println!("{:?}\n", descriptions);
    println!("{:?}\n", fields);
    println!("{:?}\n", enum_values);


    Ok(())
}