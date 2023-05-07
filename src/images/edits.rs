//! Creates an edited or extended image given an original image and a prompt.

use super::{openai_post, ApiResponseOrError, OpenAiError, ImageData, ImageFormat, ImageSize};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct ImageEdit {
    pub created: usize,
    pub data: Vec<ImageData>
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "ImageEditBuilder")]
#[builder(setter(strip_option, into))]
pub struct ImageEditRequest {
    /// The image to edit.
    /// Must be a valid PNG file, less than 4MB, and square.
    /// If mask is not provided, image must have transparency, which will be used as the mask.
    pub image: String,

    /// An additional image whose fully transparent areas (e.g. where alpha is zero) indicate where `image` should be edited.
    /// Must be a valid PNG file, less than 4MB, and have the same dimensions as `image`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into = false), default)]
    pub mask: Option<String>,

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

impl ImageEdit {
    async fn create(request: &ImageEditRequest) -> ApiResponseOrError<Self> {
        let response: Result<Self, OpenAiError> = openai_post(
            "images/edits",
            request
        ).await?;

        Ok(match response {
            Ok(image) => Ok(image),
            Err(_) => response,
        })
    }

    pub fn builder(image: &str, prompt: &str) -> ImageEditBuilder {
        ImageEditBuilder::create_empty()
            .image(image)
            .prompt(prompt)
    }
}

impl ImageEditBuilder {
    pub async fn create(self) -> ApiResponseOrError<ImageEdit> {
        ImageEdit::create(&self.build().unwrap()).await
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

        let image = ImageEdit::builder("@otter.png", "A cute baby sea otter wearing a beret")
            .mask("@mask.png".to_owned())
            .n(2)
            .size(ImageSize::Large)
            .create()
            .await
            .unwrap()
            .unwrap();

        assert!(image.data.len() > 0)
    }
}