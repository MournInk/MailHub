use crate::types::{AIClassification, AIConfig, AIProvider, Category, Email};
use anyhow::Result;
use regex::Regex;
use reqwest::Client;
use serde_json::json;

pub struct AIClassifier {
    config: AIConfig,
    client: Client,
}

impl AIClassifier {
    pub fn new(config: AIConfig) -> Self {
        Self {
            config,
            client: Client::new(),
        }
    }

    pub async fn classify_email(&self, email: &Email) -> Result<AIClassification> {
        if !self.config.enabled {
            return Ok(AIClassification {
                category: Category::Normal,
                verification_code: None,
                verification_link: None,
                should_notify: false,
            });
        }

        // First, try to extract verification code and links with regex
        let verification_code = Self::extract_verification_code(&email.body);
        let verification_link = Self::extract_verification_link(&email.body);

        // Then use AI to classify
        let category = self.classify_with_ai(&email.subject, &email.body).await?;

        let should_notify = matches!(category, Category::Important | Category::Verification);

        Ok(AIClassification {
            category,
            verification_code,
            verification_link,
            should_notify,
        })
    }

    async fn classify_with_ai(&self, subject: &str, body: &str) -> Result<Category> {
        let prompt = format!(
            "Classify this email into one of these categories: marketing, important, verification, or normal.\n\nSubject: {}\n\nBody preview: {}\n\nRespond with just the category name.",
            subject,
            &body[..body.len().min(500)]
        );

        let response = match self.config.provider {
            AIProvider::OpenAI => self.call_openai_api(&prompt).await?,
            AIProvider::Anthropic => self.call_anthropic_api(&prompt).await?,
            AIProvider::Gemini => self.call_gemini_api(&prompt).await?,
        };

        let category_str = response.to_lowercase();
        let category = if category_str.contains("marketing") {
            Category::Marketing
        } else if category_str.contains("important") {
            Category::Important
        } else if category_str.contains("verification") {
            Category::Verification
        } else {
            Category::Normal
        };

        Ok(category)
    }

    async fn call_openai_api(&self, prompt: &str) -> Result<String> {
        let endpoint = self.config.api_endpoint.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://api.openai.com/v1/chat/completions");

        let model = self.config.model.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("gpt-3.5-turbo");

        let response = self.client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&json!({
                "model": model,
                "messages": [
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 50,
                "temperature": 0.3
            }))
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        let content = data["choices"][0]["message"]["content"]
            .as_str()
            .unwrap_or("normal")
            .to_string();

        Ok(content)
    }

    async fn call_anthropic_api(&self, prompt: &str) -> Result<String> {
        let endpoint = self.config.api_endpoint.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("https://api.anthropic.com/v1/messages");

        let model = self.config.model.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("claude-3-haiku-20240307");

        let response = self.client
            .post(endpoint)
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&json!({
                "model": model,
                "messages": [
                    {"role": "user", "content": prompt}
                ],
                "max_tokens": 50
            }))
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        let content = data["content"][0]["text"]
            .as_str()
            .unwrap_or("normal")
            .to_string();

        Ok(content)
    }

    async fn call_gemini_api(&self, prompt: &str) -> Result<String> {
        let model = self.config.model.as_ref()
            .map(|s| s.as_str())
            .unwrap_or("gemini-pro");

        let endpoint = self.config.api_endpoint.as_ref()
            .map(|s| format!("{}?key={}", s, self.config.api_key))
            .unwrap_or_else(|| format!(
                "https://generativelanguage.googleapis.com/v1/models/{}:generateContent?key={}",
                model, self.config.api_key
            ));

        let response = self.client
            .post(&endpoint)
            .json(&json!({
                "contents": [{
                    "parts": [{"text": prompt}]
                }]
            }))
            .send()
            .await?;

        let data: serde_json::Value = response.json().await?;
        let content = data["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .unwrap_or("normal")
            .to_string();

        Ok(content)
    }

    fn extract_verification_code(text: &str) -> Option<String> {
        // Common verification code patterns
        let patterns = vec![
            Regex::new(r"(?i)code[:：\s]+([A-Z0-9]{4,8})").ok()?,
            Regex::new(r"(?i)verification[:：\s]+([A-Z0-9]{4,8})").ok()?,
            Regex::new(r"(?i)([0-9]{4,8})\s+is\s+your\s+code").ok()?,
            Regex::new(r"\b([0-9]{6})\b").ok()?, // 6-digit codes are common
        ];

        for pattern in patterns {
            if let Some(captures) = pattern.captures(text) {
                if let Some(code) = captures.get(1) {
                    return Some(code.as_str().to_string());
                }
            }
        }

        None
    }

    fn extract_verification_link(text: &str) -> Option<String> {
        let link_pattern = Regex::new(
            r#"https?://[^\s<>"]+(?:verify|confirm|activate|validation)[^\s<>"]*"#
        ).ok()?;

        if let Some(captures) = link_pattern.captures(text) {
            return Some(captures.get(0)?.as_str().to_string());
        }

        None
    }
}
