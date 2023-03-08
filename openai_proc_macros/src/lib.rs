mod openapi;

extern crate proc_macro;
use openapi::OpenApi;
use proc_macro::TokenStream;
use std::fs;

// NOTES
// * all components are objects
// * some don't have the type field, for some reason

#[proc_macro]
pub fn generate_from_schema(_item: TokenStream) -> TokenStream {
    let file = fs::read("openapi.yaml").expect("failed to read 'openapi.yaml'");
    let openapi_schema: OpenApi =
        serde_yaml::from_slice(&file).expect("failed to deserialize 'openapi.yaml'");

    dbg!(openapi_schema);

    //    for (component_name, component_schema) in openapi_schema.components.schemas {
    //        dbg!(component_name, component_schema);
    //    }

    // placeholder return value
    _item
}
