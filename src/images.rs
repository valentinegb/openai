//! Given a prompt and/or an input image, the model will generate a new image.
//! Related guide: [Image generation](https://platform.openai.com/docs/guides/images)

use super::{openai_post, ApiResponseOrError, OpenAiError};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

pub mod edits;
pub mod variations;

#[derive(Deserialize, Clone)]
pub struct Image {
    pub created: usize,
    pub data: Vec<ImageData>
}

#[derive(Deserialize, Clone)]
pub struct ImageData {
    pub url: String
}

#[derive(Serialize, Clone, Default, Debug)]
pub enum ImageSize {
    #[serde(rename = "256x256")]
    Small,
    #[serde(rename = "512x512")]
    Medium,
    #[default]
    #[serde(rename = "1024x1024")]
    Large,
}

#[derive(Serialize, Clone, Default, Debug)]
pub enum ImageFormat {
    #[default]
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "b64_json")]
    Base64Json,
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "ImageBuilder")]
#[builder(setter(strip_option, into))]
pub struct ImageRequest {
    /// A text description of the desired image(s).
    /// The maximum length is 1000 characters.
    pub prompt: String,

    /// The number of images to generate.
    /// Must be between 1 and 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into = false), default)]
    pub n: Option<u16>,

    /// The size of the generated images.
    /// Must be one of `256x256`, `512x512`, or `1024x1024`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub size: Option<ImageSize>,

    /// The format in which the generated images are returned.
    /// Must be one of `url` or `b64_json`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub response_format: Option<ImageFormat>,

    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub user: Option<String>,
}

impl Image {
    async fn create(request: &ImageRequest) -> ApiResponseOrError<Self> {
        let response: Result<Self, OpenAiError> = openai_post(
            "images/generations",
            request
        ).await?;

        Ok(match response {
            Ok(image) => Ok(image),
            Err(_) => response,
        })
    }

    pub fn builder(prompt: &str) -> ImageBuilder {
        ImageBuilder::create_empty()
            .prompt(prompt)
    }
}

impl ImageBuilder {
    pub async fn create(self) -> ApiResponseOrError<Image> {
        Image::create(&self.build().unwrap()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_key;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    #[ignore]
    async fn image() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());

        let image = Image::builder("A cute baby sea otter")
            .n(2)
            .size(ImageSize::Large)
            .create()
            .await
            .unwrap()
            .unwrap();

        assert!(image.data.len() > 0)
    }
}