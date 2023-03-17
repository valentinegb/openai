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

    for group in openapi_schema.meta.groups {
        if let Some(warning) = group.warning {
            if warning.title.to_lowercase().contains("deprecated") {
                continue;
            }
        }

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
            let fn_documentation = operation.summary.replace("\n", "\n/// ");
            let fn_name = operation.operation_id.to_snake_case();
            let fn_return_type = match operation.responses.ok.content.json.schema {
                Schema::Reference { reference } => reference.split('/').last().unwrap().to_string(),
                Schema::Type { r#type } => r#type,
            };

            functions += &format!(
                "/// {fn_documentation}
                pub async fn {fn_name}({}) -> ResponseOrError<{fn_return_type}> {{
                    Client::new()
                        .get(\"{}{}\")
                        .header(
                            \"Authorization\",
                            format!(\"Bearer {{}}\", API_KEY.lock().unwrap()),
                        )
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap()
                }}"
            );
        }

        let mod_documentation = group.description.replace("\n", "\n/// ");
        let mod_name = group.id.to_snake_case();

        result += &format!(
            "/// {mod_documentation}
            pub mod {mod_name} {{
                use crate::{{components::*, *}};
                use reqwest::Client;

                {functions}
            }}"
        );
    }

    dbg!(&result);

    result.parse().unwrap()
}
