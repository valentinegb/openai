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
    let base_url = &openapi_schema.servers.first().unwrap().url;

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
                Schema::Type { r#type } => r#type.unwrap().to_string(),
            };

            let mut fn_parameters = String::new();
            for parameter in operation.parameters {
                fn_parameters += &format!(
                    "{}: {},",
                    parameter.name,
                    match parameter.schema {
                        Schema::Reference { reference } =>
                            reference.split('/').last().unwrap().to_string(),
                        Schema::Type { r#type } => r#type.unwrap().to_string(),
                    }
                );
            }

            // TODO: the format!() macro in .get() doesn't work and I have no idea why
            functions += &format!(
                "/// {fn_documentation}
                pub async fn {fn_name}(
                    {fn_parameters}
                ) -> ResponseOrError<{fn_return_type}> {{
                    Client::new()
                        .get(format!(\"{base_url}{path}\"))
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

    let mut components = String::new();

    for (component_name, component) in openapi_schema.components.schemas {
        components += &format!(
            "#[derive(Deserialize, Serialize, Builder, Debug, Clone)]
            #[builder(setter(into))]
            pub struct {component_name} {{}}"
        );
    }

    result += &format!(
        "pub mod components {{
            use derive_builder::Builder;
            use serde::{{Deserialize, Serialize}};

            {components}
        }}"
    );

    println!("{}", &result);

    result.parse().unwrap()
}
