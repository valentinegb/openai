extern crate openai_proc_macros;
use openai_proc_macros::generate_from_schema;

generate_from_schema!();

// Everything between --[ and ]-- is an example of how part of the yaml should be deserialized.
// It's good to know what exactly your end goal is!
// --[

///// List and describe the various models available in the API. You can refer to the [Models](/docs/models) documentation to understand what models are available and the differences between them.
//pub mod models {
//    use crate::{components::*, *};
//    use reqwest::Client;
//
//    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
//    pub async fn list_models() -> ResponseOrError<ListModelsResponse> {
//        Client::new()
//            .get("https://api.openai.com/v1/models")
//            .header(
//                "Authorization",
//                format!("Bearer {}", API_KEY.lock().unwrap()),
//            )
//            .send()
//            .await
//            .unwrap()
//            .json()
//            .await
//            .unwrap()
//    }
//
//    /// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
//    ///
//    /// ## Parameters
//    /// * `model` - The ID of the model to use for this request
//    pub async fn retrieve_model(model: &str) -> ResponseOrError<Model> {
//        Client::new()
//            .get(format!("https://api.openai.com/v1/models/{model}"))
//            .header(
//                "Authorization",
//                format!("Bearer {}", API_KEY.lock().unwrap()),
//            )
//            .send()
//            .await
//            .unwrap()
//            .json()
//            .await
//            .unwrap()
//    }
//
//    /// Delete a fine-tuned model. You must have the Owner role in your organization.
//    ///
//    /// ## Parameters
//    /// * `model` - The model to delete
//    pub async fn delete_model(model: &str) -> ResponseOrError<DeleteModelResponse> {
//        Client::new()
//            .delete(format!("https://api.openai.com/v1/models/{model}"))
//            .header(
//                "Authorization",
//                format!("Bearer {}", API_KEY.lock().unwrap()),
//            )
//            .send()
//            .await
//            .unwrap()
//            .json()
//            .await
//            .unwrap()
//    }
//}
//
//pub mod components {
//    use derive_builder::Builder;
//    use serde::{Deserialize, Serialize};
//
//    #[derive(Deserialize, Serialize, Builder, Debug, Clone)]
//    #[builder(setter(into))]
//    pub struct Model {
//        pub id: String,
//        pub object: String,
//        pub created: i64,
//        pub owned_by: String,
//    }
//
//    #[derive(Deserialize, Serialize, Builder, Debug, Clone)]
//    #[builder(setter(into))]
//    pub struct ListModelsResponse {
//        pub object: String,
//        pub data: Vec<Model>,
//    }
//
//    #[derive(Deserialize, Serialize, Builder, Debug, Clone)]
//    #[builder(setter(into))]
//    pub struct DeleteModelResponse {
//        pub id: String,
//        pub object: String,
//        pub deleted: bool,
//    }
//}
// ]--

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

static API_KEY: Mutex<String> = Mutex::new(String::new());

pub fn set_key(value: String) {
    *API_KEY.lock().unwrap() = value;
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum ResponseOrError<T> {
    Response(T),
    Error { error: ApiError },
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ApiError {
    pub code: Option<String>,
    pub message: String,
    pub param: Option<String>,
    pub r#type: String,
}

#[cfg(test)]
mod tests {
    use crate::set_key;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn models() {
        use crate::models;

        dotenv().unwrap();
        set_key(std::env::var("OPENAI_KEY").unwrap());

        dbg!(models::list_models().await);
        dbg!(models::retrieve_model("text-davinci-003").await);
        dbg!(models::delete_model("made-up-model").await); // meant to return an `ApiError`
    }
}
