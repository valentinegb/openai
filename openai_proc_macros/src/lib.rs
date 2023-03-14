mod openapi;

extern crate proc_macro;
use openapi::*;
use proc_macro::TokenStream;
use std::{collections::HashMap, fs};

fn organize_operation(
    groups: &mut HashMap<String, HashMap<String, Operation>>,
    operation_kind: &str,
    path: &str,
    operation: Operation,
) {
    let operation_and_path = format!("{operation_kind} {path}");

    if let Some(group) = groups.get_mut(&operation.meta.group) {
        group.insert(operation_and_path, operation);
    } else {
        groups.insert(
            operation.meta.group.clone(),
            [(operation_and_path, operation)].into(),
        );
    }
}

#[proc_macro]
pub fn generate_from_schema(_item: TokenStream) -> TokenStream {
    let mut result = String::new();

    let file = fs::read("openapi.yaml").expect("failed to read 'openapi.yaml'");
    let openapi_schema: OpenApi =
        serde_yaml::from_slice(&file).expect("failed to deserialize 'openapi.yaml'");

    let mut groups: HashMap<String, HashMap<String, Operation>> = HashMap::new();

    for (path, item) in openapi_schema.paths {
        if let Some(operation) = item.get {
            organize_operation(&mut groups, "GET", &path, operation);
        }

        if let Some(operation) = item.post {
            organize_operation(&mut groups, "POST", &path, operation);
        }

        if let Some(operation) = item.delete {
            organize_operation(&mut groups, "DELETE", &path, operation);
        }
    }

    for group in groups {
        // TODO: create modules for each group containing functions for their operations
    }

    dbg!(&result);

    result.parse().unwrap()
}
