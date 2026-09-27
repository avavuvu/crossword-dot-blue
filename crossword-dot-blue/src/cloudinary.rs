use std::{env, fmt, sync::LazyLock};

use serde::Deserialize;
use sha1::{Digest, Sha1};

pub struct Config {
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

static CONFIG: LazyLock<Option<Config>> = LazyLock::new(|| {
    match (
        env::var("CLOUDINARY_CLOUD_NAME"),
        env::var("CLOUDINARY_API_KEY"),
        env::var("CLOUDINARY_API_SECRET"),
    ) {
        (Ok(cloud_name), Ok(api_key), Ok(api_secret)) => Some(Config { cloud_name, api_key, api_secret }),
        _ => None,
    }
});

pub fn init() {
    LazyLock::force(&CONFIG);
}

fn config() -> Option<&'static Config> {
    CONFIG.as_ref()
}

pub fn is_configured() -> bool {
    config().is_some()
}

pub fn url(public_id: &str, transform: &str) -> String {
    let cloud_name = config().map(|config| config.cloud_name.as_str()).unwrap_or("demo");
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
            Error::Request(error) => write!(f, "cloudinary request: {error}"),
            Error::Rejected(message) => write!(f, "cloudinary rejected the upload: {message}"),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::Request(error)
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

fn sign(parameters: &[(&str, &str)], secret: &str) -> String {
    let mut sorted: Vec<&(&str, &str)> = parameters.iter().collect();
    sorted.sort_by(|left, right| left.0.cmp(right.0));
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

    let parameters = [
        ("invalidate", "true"),
        ("overwrite", "true"),
        ("public_id", public_id),
        ("timestamp", timestamp.as_str()),
    ];
    let signature = sign(&parameters, &config.api_secret);

    let file = reqwest::multipart::Part::bytes(bytes)
        .file_name("avatar")
        .mime_str(content_type)?;

    let mut form = reqwest::multipart::Form::new()
        .text("api_key", config.api_key.clone())
        .text("signature", signature)
        .part("file", file);
    for (name, value) in parameters {
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
