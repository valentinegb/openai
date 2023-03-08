// These structures are for the explicit purpose of deserializing the OpenAI API schema, not any OpenAPI schema.
// I'm making my own structures and such instead of using a pre-existing library because
// they're a bit overcomplicated for what they're needed for.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct OpenApi {
    pub servers: Vec<Server>,
    pub components: Components,
}

#[derive(Deserialize, Debug)]
pub struct Server {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Components {
    pub schemas: HashMap<String, Schema>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Schema {
    Tagged(TaggedSchema),
    Object(Object),
    Other(serde_yaml::Value),
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub enum TaggedSchema {
    Null,
    Boolean,
    Object(Object),
    Array,
    Number,
    String,
    Integer,
}

#[derive(Deserialize, Debug)]
pub struct Object {
    pub properties: HashMap<String, Schema>,
    #[serde(default)]
    pub required: Vec<String>,
}
