use std::{env, sync::LazyLock};

pub struct Config {
    pub site_url: String,
    admin_emails: Vec<String>,
}

static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let site_url = env::var("SITE_URL").expect("SITE_URL must be set");
    let admin_emails = env::var("ADMIN_EMAILS")
        .unwrap_or_default()
        .split(',')
        .map(|email| email.trim().to_ascii_lowercase())
        .filter(|email| !email.is_empty())
        .collect();

    Config { site_url: site_url.trim_end_matches('/').to_string(), admin_emails }
});

pub fn init() {
    LazyLock::force(&CONFIG);
}

pub fn get() -> &'static Config {
    &CONFIG
}

pub fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

pub fn is_admin_email(email: &str) -> bool {
    let email = normalize_email(email);
    get().admin_emails.iter().any(|admin| *admin == email)
}
