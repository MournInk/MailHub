use crate::types::{Email, EmailAccount, EmailAddress};
use anyhow::Result;

pub struct EmailClient;

impl EmailClient {
    pub async fn fetch_emails(account: &EmailAccount) -> Result<Vec<Email>> {
        // For demonstration, return mock emails
        // In production, this would implement actual IMAP/POP3/OAuth2
        Ok(Self::generate_demo_emails(&account.id))
    }

    fn generate_demo_emails(account_id: &str) -> Vec<Email> {
        vec![
            Email {
                id: uuid::Uuid::new_v4().to_string(),
                account_id: account_id.to_string(),
                subject: "Welcome to MailHub!".to_string(),
                from: EmailAddress {
                    name: Some("MailHub Team".to_string()),
                    address: "team@mailhub.app".to_string(),
                },
                to: vec![EmailAddress {
                    name: None,
                    address: "user@example.com".to_string(),
                }],
                cc: None,
                bcc: None,
                date: chrono::Utc::now().to_rfc2822(),
                body: "Thank you for using MailHub! This is a demo email to showcase the email management capabilities.".to_string(),
                html_body: Some("<p>Thank you for using <strong>MailHub</strong>! This is a demo email to showcase the email management capabilities.</p>".to_string()),
                attachments: None,
                is_read: false,
                is_starred: false,
                labels: None,
                ai_classification: None,
            },
            Email {
                id: uuid::Uuid::new_v4().to_string(),
                account_id: account_id.to_string(),
                subject: "Your verification code: 123456".to_string(),
                from: EmailAddress {
                    name: Some("Security".to_string()),
                    address: "security@example.com".to_string(),
                },
                to: vec![EmailAddress {
                    name: None,
                    address: "user@example.com".to_string(),
                }],
                cc: None,
                bcc: None,
                date: chrono::Utc::now().to_rfc2822(),
                body: "Your verification code is: 123456. Please use it to verify your account.".to_string(),
                html_body: Some("<p>Your verification code is: <strong>123456</strong>. Please use it to verify your account.</p>".to_string()),
                attachments: None,
                is_read: false,
                is_starred: false,
                labels: None,
                ai_classification: None,
            },
        ]
    }
}

pub async fn send_email(
    _account: &EmailAccount,
    _to: &str,
    _subject: &str,
    _body: &str,
) -> Result<()> {
    // For demonstration, just return success
    // In production, this would implement actual SMTP sending
    println!("Email sent successfully (demo mode)");
    Ok(())
}
