use std::sync::LazyLock;

use boutique::cloudinary::Cloudinary;
pub use boutique::cloudinary::Error;

static CLIENT: LazyLock<Option<Cloudinary>> = LazyLock::new(Cloudinary::from_env);

pub fn init() {
    LazyLock::force(&CLIENT);
}

pub fn is_configured() -> bool {
    CLIENT.is_some()
}

pub fn url(public_id: &str, transform: &str) -> String {
    match CLIENT.as_ref() {
        Some(client) => client.url(public_id, transform),
        None => format!("https://res.cloudinary.com/demo/image/upload/{transform}/{public_id}"),
    }
}

pub async fn upload(bytes: Vec<u8>, public_id: &str, content_type: &str) -> Result<String, UploadError> {
    let client = CLIENT.as_ref().ok_or(UploadError::NotConfigured)?;
    client.upload(bytes, public_id, content_type).await.map_err(UploadError::Cloudinary)
}

#[derive(Debug)]
pub enum UploadError {
    NotConfigured,
    Cloudinary(Error),
}

impl std::fmt::Display for UploadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UploadError::NotConfigured => f.write_str("cloudinary is not configured"),
            UploadError::Cloudinary(error) => error.fmt(f),
        }
    }
}
