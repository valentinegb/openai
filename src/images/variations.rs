//! Creates a variation of a given image.

use super::{openai_post, ApiResponseOrError, OpenAiError, ImageData, ImageFormat, ImageSize};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct ImageVariation {
    pub created: usize,
    pub data: Vec<ImageData>
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "ImageVariationBuilder")]
#[builder(setter(strip_option, into))]
pub struct ImageVariationRequest {
    /// The image to use as the basis for the variation(s).
    /// Must be a valid PNG file, less than 4MB, and square.
    pub image: String,

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

impl ImageVariation {
    async fn create(request: &ImageVariationRequest) -> ApiResponseOrError<Self> {
        let response: Result<Self, OpenAiError> = openai_post(
            "images/variations",
            request
        ).await?;

        Ok(match response {
            Ok(image) => Ok(image),
            Err(_) => response,
        })
    }

    pub fn builder(image: &str) -> ImageVariationBuilder {
        ImageVariationBuilder::create_empty()
            .image(image)
    }
}

impl ImageVariationBuilder {
    pub async fn create(self) -> ApiResponseOrError<ImageVariation> {
        ImageVariation::create(&self.build().unwrap()).await
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

        let image = ImageVariation::builder("@otter.png")
            .n(2)
            .size(ImageSize::Large)
            .create()
            .await
            .unwrap()
            .unwrap();

        assert!(image.data.len() > 0)
    }
}