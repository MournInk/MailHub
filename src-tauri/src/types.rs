use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAccount {
    pub id: String,
    pub name: String,
    pub email: String,
    pub display_name: Option<String>,
    pub tags: Option<Vec<String>>,
    pub protocol: Protocol,
    pub provider: Option<Provider>,
    pub config: AccountConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    Imap,
    Pop3,
    OAuth2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Provider {
    Gmail,
    Outlook,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountConfig {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub oauth_token: Option<String>,
    pub refresh_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub id: String,
    pub account_id: String,
    pub subject: String,
    pub from: EmailAddress,
    pub to: Vec<EmailAddress>,
    pub cc: Option<Vec<EmailAddress>>,
    pub bcc: Option<Vec<EmailAddress>>,
    pub date: String,
    pub body: String,
    pub html_body: Option<String>,
    pub attachments: Option<Vec<Attachment>>,
    pub is_read: bool,
    pub is_starred: bool,
    pub labels: Option<Vec<String>>,
    pub ai_classification: Option<AIClassification>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailAddress {
    pub name: Option<String>,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: String,
    pub filename: String,
    pub mime_type: String,
    pub size: u64,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIClassification {
    pub category: Category,
    pub verification_code: Option<String>,
    pub verification_link: Option<String>,
    pub should_notify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    Marketing,
    Important,
    Verification,
    Normal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub enabled: bool,
    pub provider: AIProvider,
    pub api_key: String,
    pub api_endpoint: Option<String>,
    pub model: Option<String>,
    pub auto_delete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Gemini,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub notifications: bool,
    pub ai_config: Option<AIConfig>,
    pub theme: Theme,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}
