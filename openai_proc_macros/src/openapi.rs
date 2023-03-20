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
    #[serde(rename = "x-oaiMeta")]
    pub(crate) meta: OpenAiOpenApiMeta,
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
    #[serde(rename = "operationId")]
    pub(crate) operation_id: String,
    #[serde(default)]
    pub(crate) deprecated: bool,
    pub(crate) summary: String,
    #[serde(default)]
    pub(crate) parameters: Vec<Parameter>,
    pub(crate) responses: Responses,
    #[serde(rename = "x-oaiMeta")]
    pub(crate) meta: OpenAiOperationMeta,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Parameter {
    pub(crate) name: String,
    pub(crate) r#in: ParameterLocation,
    pub(crate) description: String,
    pub(crate) required: bool,
    #[serde(default)]
    pub(crate) deprecated: bool,
    pub(crate) schema: Schema,
}

#[derive(Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub(crate) enum ParameterLocation {
    Path,
    Query,
    Header,
    Cookie,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Responses {
    #[serde(rename = "200")]
    pub(crate) ok: Response,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Response {
    pub(crate) content: ResponseContent,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ResponseContent {
    #[serde(rename = "application/json")]
    pub(crate) json: MediaType,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct MediaType {
    pub(crate) schema: Schema,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub(crate) enum Schema {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Type {
        r#type: Option<SchemaType>,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) enum SchemaType {
    Boolean,
    Object,
    Array,
    Number,
    String,
    Integer,
}

impl std::fmt::Display for SchemaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Boolean => "bool",
                Self::Object => "HashMap<String, serde_json::Value>",
                Self::Array => "Vec<serde_json::Value>",
                Self::Number => "f64",
                Self::String => "String",
                Self::Integer => "i64",
            }
        )
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OpenAiOperationMeta {
    pub(crate) group: String,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Components {
    pub(crate) schemas: HashMap<String, Schema>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OpenAiOpenApiMeta {
    pub(crate) groups: Vec<Group>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Group {
    pub(crate) id: String,
    pub(crate) description: String,
    pub(crate) warning: Option<GroupWarning>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct GroupWarning {
    pub(crate) title: String,
}
