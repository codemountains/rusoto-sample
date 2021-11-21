use std::collections::HashMap;
use rusoto_core::Region;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, QueryInput, Condition};

#[tokio::main]
async fn main() {
    let client = DynamoDbClient::new(Region::ApNortheast1);

    // scan
    // let scan_input = ScanInput {
    //     table_name: "Mountains".to_string(),
    //     ..Default::default()
    // };
    //
    // match client.scan(scan_input).await {
    //     Ok(result) => {
    //         match result.items {
    //             Some(items) => println!("{:?}", items[2]),
    //             None => print!("None!")
    //         }
    //         // print!("{:?}", result.items[0]);
    //     },
    //     Err(error) => {
    //         panic!("Error: {:?}", error);
    //     },
    // };

    let mut list: Vec<AttributeValue> = Vec::new();
    list.push(AttributeValue {
        n: Some(String::from("1000")),
        ..Default::default()
    });

    let attr_value_list: Option<Vec<AttributeValue>> = Some(list);

    let mut key: HashMap<String, Condition> = HashMap::new();
    key.insert(String::from("Id"), Condition {
        attribute_value_list: attr_value_list,
        comparison_operator: "EQ".to_string()
    });

    let key_conditions: Option<HashMap<String, Condition>> = Some(key);

    let query = QueryInput {
        key_conditions,
        table_name: "Mountains".to_string(),
        ..Default::default()
    };

    match client.query(query).await {
        Ok(result) => {
            match result.items {
                Some(items) => {
                    for item in items {
                        // println!("{:?}", item);

                        // Id
                        // match item.get(&*"Id") {
                        //     Some(attr) => {
                        //         match &attr.n {
                        //             Some(id) => println!("{}", id),
                        //             _ => {}
                        //         }
                        //     },
                        //     _ => {}
                        // }

                        match item.get(&*"DataType") {
                            Some(attr) => {
                                match &attr.s {
                                    Some(data_type) => {
                                        match data_type.as_str() {
                                            "Name" => {
                                                match item.get(&*"DataValue") {
                                                    Some(value_attr) => {
                                                        match &value_attr.s {
                                                            Some(data_value) => {
                                                                println!("{}", data_value);
                                                            },
                                                            _ => {}
                                                        }
                                                    }
                                                    _ => {}
                                                }
                                            }
                                            _ => {}
                                        }
                                    },
                                    _ => {}
                                }
                            },
                            _ => {}
                        }
                    }
                },
                None => print!("UnMatch!")
            };
        },
        Err(error) => {
            print!("Error: {:?}", error);
        },
    };
}
