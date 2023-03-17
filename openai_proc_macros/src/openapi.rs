// These structures are for the explicit purpose of deserializing the OpenAI API schema, not any OpenAPI schema.
// I'm making my own structures and such instead of using a pre-existing library because
// they're a bit overcomplicated for what they're needed for.

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OpenApi {
    pub(crate) paths: HashMap<String, PathItem>,
    #[serde(rename = "x-oaiMeta")]
    pub(crate) meta: OpenAiOpenApiMeta,
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
    pub(crate) responses: Responses,
    #[serde(rename = "x-oaiMeta")]
    pub(crate) meta: OpenAiOperationMeta,
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
#[serde(rename_all = "PascalCase")]
pub(crate) enum Schema {
    Reference {
        #[serde(rename = "$ref")]
        reference: String,
    },
    Type {
        r#type: String,
    },
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct OpenAiOperationMeta {
    pub(crate) group: String,
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
