use once_cell::sync::Lazy;
use reqwest::{header::AUTHORIZATION, Client, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Mutex;

pub mod chat;
pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod models;
pub mod moderations;

static BASE_URL: Lazy<Mutex<String>> =
    Lazy::new(|| Mutex::new("https://api.openai.com/v1/".to_string()));

static API_KEY: Mutex<String> = Mutex::new(String::new());

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum ApiType {
    OpenAi,
    Azure,
}

static API_TYPE: Mutex<ApiType> = Mutex::new(ApiType::OpenAi);

#[derive(Deserialize, Debug, Clone)]
pub struct OpenAiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

impl std::fmt::Display for OpenAiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OpenAiError {}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err { error: OpenAiError },
}

#[derive(Deserialize, Clone, Copy)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

type ApiResponseOrError<T> = Result<Result<T, OpenAiError>, reqwest::Error>;

async fn openai_request<F, T>(method: Method, route: &str, builder: F) -> ApiResponseOrError<T>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
    T: DeserializeOwned,
{
    let client = Client::new();
    let mut url = BASE_URL.lock().unwrap().to_owned() + route;
    let api_type = *API_TYPE.lock().unwrap();
    if api_type == ApiType::Azure {
        url += "?api-version=2023-03-15-preview";
    }

    let mut request = client.request(method, url);

    request = builder(request);

    let key = API_KEY.lock().unwrap();
    if api_type == ApiType::OpenAi {
        request = request.header(AUTHORIZATION, format!("Bearer {}", key))
    } else {
        request = request.header("api-key", key.to_string())
    }
    let api_response: ApiResponse<T> = request.send().await?.json().await?;

    match api_response {
        ApiResponse::Ok(t) => Ok(Ok(t)),
        ApiResponse::Err { error } => Ok(Err(error)),
    }
}

async fn openai_get<T>(route: &str) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request(Method::GET, route, |request| request).await
}

async fn openai_post<J, T>(route: &str, json: &J) -> ApiResponseOrError<T>
where
    J: Serialize + ?Sized,
    T: DeserializeOwned,
{
    openai_request(Method::POST, route, |request| request.json(json)).await
}

/// Sets the key for all OpenAI API functions.
///
/// ## Examples
///
/// Use environment variable `OPENAI_KEY` defined from `.env` file:
///
/// ```rust
/// use openai::set_key;
/// use dotenvy::dotenv;
/// use std::env;
///
/// dotenv().ok();
/// set_key(env::var("OPENAI_KEY").unwrap());
/// ```
pub fn set_key(value: String) {
    *API_KEY.lock().unwrap() = value;
}

/// Sets the API type that is used
///
/// ## Examples
///
/// Use a custom Azure OpenAI endpoint
///
/// ```rust
/// use openai::{set_base_url,ApiType};
///
/// set_base_url(ApiType::Azure);
/// ```
pub fn set_api_type(value: ApiType) {
    *API_TYPE.lock().unwrap() = value;
}

/// Sets the base URL for all OpenAI API functions.
///
/// ## Examples
///
/// Use a custom Azure OpenAI endpoint
///
/// ```rust
/// use openai::set_base_url;
///
/// set_base_url("https://docs-test-001.openai.azure.com/openai/deployments/my-own-gpt-3.5");
/// ```
pub fn set_base_url(value: String) {
    *BASE_URL.lock().unwrap() = value;
}
