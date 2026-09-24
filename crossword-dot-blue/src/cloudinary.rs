use std::{env, fmt, sync::OnceLock};

use serde::Deserialize;
use sha1::{Digest, Sha1};

pub struct Config {
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

static CONFIG: OnceLock<Option<Config>> = OnceLock::new();

pub fn init() {
    let config = match (
        env::var("CLOUDINARY_CLOUD_NAME"),
        env::var("CLOUDINARY_API_KEY"),
        env::var("CLOUDINARY_API_SECRET"),
    ) {
        (Ok(cloud_name), Ok(api_key), Ok(api_secret)) => Some(Config { cloud_name, api_key, api_secret }),
        _ => None,
    };
    CONFIG.set(config).ok();
}

fn config() -> Option<&'static Config> {
    CONFIG.get().and_then(Option::as_ref)
}

pub fn is_configured() -> bool {
    config().is_some()
}

pub fn url(public_id: &str, transform: &str) -> String {
    let cloud_name = config().map(|c| c.cloud_name.as_str()).unwrap_or("demo");
    format!("https://res.cloudinary.com/{cloud_name}/image/upload/{transform}/{public_id}")
}

#[derive(Debug)]
pub enum Error {
    NotConfigured,
    Request(reqwest::Error),
    Rejected(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NotConfigured => f.write_str("cloudinary is not configured"),
            Error::Request(e) => write!(f, "cloudinary request: {e}"),
            Error::Rejected(message) => write!(f, "cloudinary rejected the upload: {message}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Request(e)
    }
}

#[derive(Deserialize)]
struct UploadResponse {
    public_id: Option<String>,
    error: Option<UploadError>,
}

#[derive(Deserialize)]
struct UploadError {
    message: String,
}

fn sign(params: &[(&str, &str)], secret: &str) -> String {
    let mut sorted: Vec<&(&str, &str)> = params.iter().collect();
    sorted.sort_by(|a, b| a.0.cmp(b.0));
    let joined = sorted
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join("&");
    let digest = Sha1::digest(format!("{joined}{secret}").as_bytes());
    format!("{digest:x}")
}

pub async fn upload(bytes: Vec<u8>, public_id: &str, content_type: &str) -> Result<String, Error> {
    let config = config().ok_or(Error::NotConfigured)?;
    let timestamp = chrono::Utc::now().timestamp().to_string();

    let params = [
        ("invalidate", "true"),
        ("overwrite", "true"),
        ("public_id", public_id),
        ("timestamp", timestamp.as_str()),
    ];
    let signature = sign(&params, &config.api_secret);

    let file = reqwest::multipart::Part::bytes(bytes)
        .file_name("avatar")
        .mime_str(content_type)?;

    let mut form = reqwest::multipart::Form::new()
        .text("api_key", config.api_key.clone())
        .text("signature", signature)
        .part("file", file);
    for (name, value) in params {
        form = form.text(name, value.to_string());
    }

    let endpoint = format!("https://api.cloudinary.com/v1_1/{}/image/upload", config.cloud_name);
    let response: UploadResponse = reqwest::Client::new()
        .post(endpoint)
        .multipart(form)
        .send()
        .await?
        .json()
        .await?;

    match (response.public_id, response.error) {
        (Some(public_id), _) => Ok(public_id),
        (None, Some(error)) => Err(Error::Rejected(error.message)),
        (None, None) => Err(Error::Rejected("no public_id in response".to_string())),
    }
}
