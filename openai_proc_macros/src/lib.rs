mod openapi;

extern crate proc_macro;
use heck::ToSnakeCase;
use openapi::*;
use proc_macro::TokenStream;
use std::{collections::HashMap, fs};

#[proc_macro]
pub fn generate_from_schema(_item: TokenStream) -> TokenStream {
    let mut result = String::new();

    let file = fs::read("openapi.yaml").expect("failed to read 'openapi.yaml'");
    let openapi_schema: OpenApi =
        serde_yaml::from_slice(&file).expect("failed to deserialize 'openapi.yaml'");

    // groups -> modules
    for group in openapi_schema.meta.groups {
        if let Some(warning) = group.warning {
            if warning.title.to_lowercase().contains("deprecated") {
                continue;
            }
        }

        // operations -> functions
        let mut group_get_operations: HashMap<String, Operation> = HashMap::new();

        for (path, path_item) in &openapi_schema.paths {
            if let Some(operation) = &path_item.get {
                if operation.deprecated {
                    continue;
                }

                if operation.meta.group == group.id {
                    group_get_operations.insert(path.clone(), operation.clone());
                }
            }
        }

        let mut functions = String::new();

        for (path, operation) in group_get_operations {
            if let Schema::Reference { reference } = operation.responses.ok.content.json.schema {
                let component_name = reference.split('/').last();
            }

            functions += &format!(
                "/// {}\npub fn {}() {{}}",
                operation.summary.replace("\n", "\n/// "),
                operation.operation_id.to_snake_case()
            );
        }

        // modules with functions
        result += &format!(
            "/// {}\npub mod {} {{{functions}}}",
            group.description.replace("\n", "\n/// "),
            group.id.to_snake_case()
        );
    }

    dbg!(&result);

    result.parse().unwrap()
}
