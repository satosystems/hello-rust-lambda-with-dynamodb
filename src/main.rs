// https://qiita.com/shunp/items/8b11450155266d2fcfc1
#[macro_use]
extern crate lambda_runtime as lambda;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;
extern crate simple_logger;

extern crate rusoto_core;
extern crate rusoto_dynamodb;

use lambda::error::HandlerError;

use std::error::Error;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput, GetItemInput, PutItemInput, DeleteItemInput, AttributeValue};
use std::collections::HashMap;

#[derive(Deserialize, Clone)]
struct CustomEvent {
    name: String,
    address: String,
}

#[derive(Serialize, Clone)]
struct CustomOutput {
    message: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::SimpleLogger::new()
        .with_level(log::LevelFilter::Info)
        .init()
        .unwrap();
    lambda!(my_handler);

    Ok(())
}

fn my_handler(e: CustomEvent, c: lambda::Context) -> Result<CustomOutput, HandlerError> {
    if e.name == "" {
        error!("Empty name in request {}", c.aws_request_id);
        return Err(c.new_error("Empty name"));
    }

    let mut create_key: HashMap<String, AttributeValue> = HashMap::new();
    create_key.insert(String::from("name"), AttributeValue {
        s: Some(e.name),
        ..Default::default()
    });
    create_key.insert(String::from("address"), AttributeValue {
        s: Some(e.address),
        ..Default::default()
    });

    let create_serials = PutItemInput {
        item: create_key,
        table_name: String::from("rust_serverless_sample"),
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::UsEast1);

    match client.put_item(create_serials).sync() {
        Ok(result) => {
            match result.attributes {
                Some(_) => println!("some"),
                None => println!("none"),
            }
        },
        Err(error) => {
            panic!("Error: {:?}", error);
        },
    };

    let mut query_key: HashMap<String, AttributeValue> = HashMap::new();
    query_key.insert(String::from("name"), AttributeValue {
        s: Some(e.name),
        ..Default::default()
    });

    let query_serials = GetItemInput {
        key: query_key,
        table_name: String::from("rust_serverless_sample"),
        ..Default::default()
    };

    let client = DynamoDbClient::new(Region::UsEast1);

    match client.get_item(query_serials).sync() {
        Ok(result) => {
            match result.item {
                Some(_) => print!("Match!"),
                None => print!("UnMatch!")
            }

        },
        Err(error) => {
            panic!("Error: {:?}", error);
        },
    };

    Ok(CustomOutput {
        message: format!("Success"),
    })
}
