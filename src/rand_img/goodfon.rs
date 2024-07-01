use super::{ImageBytes, ImageProvider};
use crate::configuration::ImageConfig;
use regex::Regex;
use surf::http::convert::{Deserialize, Serialize};

pub type Result<T> = core::result::Result<T, Error>;

/// Error of job execution
#[derive(Debug)]
pub enum Error {
    HttpError(surf::http::Error)
}

impl From<Error> for super::Error {
    fn from(error: Error) -> Self {
        Self::GoodFonProviderError(error)
    }
}

impl From<surf::http::Error> for Error {
    fn from(error: surf::http::Error) -> Self {
        Self::HttpError(error)
    }
}

/// 
pub struct GoodFonProvider{
    config: ImageConfig
}

impl GoodFonProvider {
    /// 
    pub fn new(config: ImageConfig) -> Self{
        Self { config }
    }
}

/// 
impl ImageProvider for GoodFonProvider{
    ///
    async fn get_image(&self) -> super::Result<ImageBytes> {
        let category = &self.config.category;
        let resolution = &self.config.resolution;

        let image_bytes = download_image(category, resolution).await?;
        Ok(image_bytes)
    }
}

/// Find and download random image
async fn download_image(category: &String, resolution: &String) -> Result<ImageBytes> {
    let url = format!("https://www.goodfon.ru/mix/");

    let request_body = RequestBody::new(category, resolution);
    let request_body = serde_urlencoded::to_string(request_body).unwrap();

    let response = surf::post(url)
        .body(request_body)
        .header("Content-Type", "application/x-www-form-urlencoded; charset=UTF-8")
        .await?
        .body_json::<ResponseBody>()
        .await?;

    let mut image_url = response.result.items
        .get(1).unwrap()
        .img.to_owned();

    image_url = Regex::new(r"wallpaper/big").unwrap()
        .replace(&image_url, format!("original/{resolution}"))
        .to_string();

    image_url = Regex::new(r"\.webp").unwrap()
        .replace(&image_url, ".jpg")
        .to_string();

    let image_bytes = surf::get(image_url).await?.body_bytes().await?;

    Ok(image_bytes.into())
}

#[derive(Serialize)]
struct RequestBody {
    catalog: String,
    resolution: String
}

impl RequestBody {
    /// ctor
    pub fn new(category: &str, resolution: &str) -> Self {
        let catalog = match category {
            "landscapes" => "13", // todo: fill match arms
            _ => panic!("category wtf") 
        };

        Self { 
            catalog: catalog.to_string(), 
            resolution: resolution.to_string()
        }
    }
}


#[derive(Deserialize)]
struct ResponseBody {
    result: ResultBody
}

#[derive(Deserialize)]
struct ResultBody {
    items: Vec<ImageInfo>
}

#[derive(Deserialize)]
struct ImageInfo {
    img: String
}