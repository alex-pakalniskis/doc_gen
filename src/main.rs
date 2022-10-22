use serde_yaml;
use std::string::String;
use graphql_parser::Pos;
use graphql_parser::schema::parse_schema;
use graphql_parser::schema::Field;
use graphql_parser::schema::EnumValue;
use graphql_parser::schema::Definition::TypeDefinition;
use graphql_parser::schema::TypeDefinition::{Scalar, Object, Interface, Union, Enum, InputObject};
use graphql_parser::query::Directive;
use graphql_parser::query::Value;
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
        match def {
            TypeDefinition(Scalar(s)) => todo!(),
            TypeDefinition(Object(o)) => {
                let mut tmp_fields: Vec<Field<String>> = Vec::new();
                let mut tmp_enum_values: Vec<EnumValue<String>> = Vec::new();
                names.push(o.name);
                descriptions.push(o.description);
                for field in o.fields {
                    tmp_fields.push(field)
                }
                fields.push(tmp_fields);
                tmp_enum_values.push(EnumValue {
                    name: "None".to_string(), 
                    position: Pos{line:0,column:0},
                    description: None,
                    directives: vec![
                        Directive {
                            position: Pos{line:0,column:0},
                            name: "None".to_string(),
                            arguments: vec![("None".to_string(), Value::Null)],
                        }
                    ],
                });
                enum_values.push(tmp_enum_values);
            },
            TypeDefinition(Interface(i)) => todo!(),
            TypeDefinition(Union(u)) => todo!(),
            TypeDefinition(Enum(e)) => {
                let mut tmp_fields: Vec<Field<String>> = Vec::new();
                let mut tmp_enum_values: Vec<EnumValue<String>> = Vec::new();
                names.push(e.name);
                descriptions.push(e.description);
                // tmp_fields.push("None"); //broken
                // fields.push(tmp_fields);
                for value in e.values {
                    tmp_enum_values.push(value)
                }
                enum_values.push(tmp_enum_values);
            },
            TypeDefinition(InputObject(io)) => todo!(),
            _ => todo!(),
        }
    }

    println!("{:?}\n", names);
    println!("{:?}\n\n", names.len());
    println!("{:?}\n", descriptions);
    println!("{:?}\n\n", descriptions.len());
    println!("{:?}\n", fields);
    println!("{:?}\n\n", fields.len());
    println!("{:?}\n", enum_values);
    println!("{:?}\n\n", enum_values.len());


    Ok(())
}