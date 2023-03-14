// These structures are for the explicit purpose of deserializing the OpenAI API schema, not any OpenAPI schema.
// I'm making my own structures and such instead of using a pre-existing library because
// they're a bit overcomplicated for what they're needed for.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OpenApi {
    pub(crate) servers: Vec<Server>,
    pub(crate) paths: HashMap<String, PathItem>,
    pub(crate) components: Components,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Server {
    pub(crate) url: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct PathItem {
    pub(crate) get: Option<Operation>,
    pub(crate) post: Option<Operation>,
    pub(crate) delete: Option<Operation>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Operation {
    pub(crate) summary: String,
    #[serde(rename = "operationId")]
    pub(crate) operation_id: String,
    #[serde(default)]
    pub(crate) parameters: Vec<Parameter>,
    #[serde(rename = "requestBody")]
    pub(crate) request_body: Option<RequestBody>,
    pub(crate) responses: Responses,
    #[serde(default)]
    pub(crate) deprecated: bool,
    #[serde(rename = "x-oaiMeta")]
    pub(crate) meta: OpenAiMeta,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Parameter {
    pub(crate) name: String,
    pub(crate) r#in: String,
    pub(crate) description: String,
    pub(crate) required: bool,
    #[serde(default)]
    pub(crate) deprecated: bool,
    pub(crate) schema: Schema,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct RequestBody {
    pub(crate) description: Option<String>,
    pub(crate) content: HashMap<String, MediaType>,
    pub(crate) required: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct MediaType {
    pub(crate) schema: SchemaOrReference,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Responses {
    pub(crate) default: Option<Response>,
    #[serde(flatten)]
    pub(crate) statuses: HashMap<String, Response>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Response {
    pub(crate) description: String,
    pub(crate) content: HashMap<String, Header>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Header {
    pub(crate) schema: SchemaOrReference,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Reference {
    #[serde(rename = "$ref")]
    pub(crate) r#ref: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum SchemaOrReference {
    Reference(Reference),
    Schema(Schema),
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Components {
    pub(crate) schemas: HashMap<String, Schema>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum Schema {
    Tagged(TaggedSchema),
    Object(ObjectSchema),
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
pub(crate) enum TaggedSchema {
    Null,
    Boolean,
    Object(ObjectSchema),
    Array,
    Number,
    String,
    Integer,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ObjectSchema {
    pub(crate) properties: Option<HashMap<String, Schema>>,
    #[serde(default)]
    pub(crate) required: Vec<String>,
    #[serde(rename = "x-oaiTypeLabel")]
    pub(crate) type_label: Option<String>,
    #[serde(default)]
    pub(crate) nullable: bool,
    pub(crate) description: Option<String>,
    pub(crate) default: Option<serde_yaml::Value>,
    pub(crate) title: Option<String>, // <-- last here
    #[serde(flatten)]
    pub(crate) other: serde_yaml::Value,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OpenAiMeta {
    pub(crate) name: String,
    pub(crate) group: String,
    pub(crate) path: String,
    pub(crate) parameters: Option<String>,
    pub(crate) response: Option<String>,
    #[serde(default)]
    pub(crate) beta: bool,
}
