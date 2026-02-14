use crate::types::{AppSettings, EmailAccount, Email};
use anyhow::Result;
use serde_json;
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Store {
    data_dir: PathBuf,
    accounts: Mutex<Vec<EmailAccount>>,
    emails: Mutex<Vec<Email>>,
    settings: Mutex<AppSettings>,
}

impl Store {
    pub fn new(data_dir: PathBuf) -> Result<Self> {
        fs::create_dir_all(&data_dir)?;
        
        let accounts_path = data_dir.join("accounts.json");
        let accounts = if accounts_path.exists() {
            let data = fs::read_to_string(&accounts_path)?;
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Vec::new()
        };

        let emails_path = data_dir.join("emails.json");
        let emails = if emails_path.exists() {
            let data = fs::read_to_string(&emails_path)?;
            serde_json::from_str(&data).unwrap_or_default()
        } else {
            Vec::new()
        };

        let settings_path = data_dir.join("settings.json");
        let settings = if settings_path.exists() {
            let data = fs::read_to_string(&settings_path)?;
            serde_json::from_str(&data).unwrap_or_else(|_| AppSettings {
                notifications: true,
                ai_config: None,
                theme: crate::types::Theme::System,
            })
        } else {
            AppSettings {
                notifications: true,
                ai_config: None,
                theme: crate::types::Theme::System,
            }
        };

        Ok(Self {
            data_dir,
            accounts: Mutex::new(accounts),
            emails: Mutex::new(emails),
            settings: Mutex::new(settings),
        })
    }

    pub fn get_accounts(&self) -> Result<Vec<EmailAccount>> {
        let accounts = self.accounts.lock().unwrap();
        Ok(accounts.clone())
    }

    pub fn add_account(&self, account: EmailAccount) -> Result<()> {
        let mut accounts = self.accounts.lock().unwrap();
        accounts.push(account);
        self.save_accounts(&accounts)?;
        Ok(())
    }

    pub fn update_account(&self, id: &str, account: EmailAccount) -> Result<()> {
        let mut accounts = self.accounts.lock().unwrap();
        if let Some(pos) = accounts.iter().position(|a| a.id == id) {
            accounts[pos] = account;
            self.save_accounts(&accounts)?;
        }
        Ok(())
    }

    pub fn delete_account(&self, id: &str) -> Result<()> {
        let mut accounts = self.accounts.lock().unwrap();
        accounts.retain(|a| a.id != id);
        self.save_accounts(&accounts)?;
        Ok(())
    }

    pub fn get_emails(&self) -> Result<Vec<Email>> {
        let emails = self.emails.lock().unwrap();
        Ok(emails.clone())
    }

    pub fn add_email(&self, email: Email) -> Result<()> {
        let mut emails = self.emails.lock().unwrap();
        emails.insert(0, email);
        self.save_emails(&emails)?;
        Ok(())
    }

    pub fn add_emails(&self, new_emails: Vec<Email>) -> Result<()> {
        let mut emails = self.emails.lock().unwrap();
        for email in new_emails {
            if !emails.iter().any(|e| e.id == email.id) {
                emails.insert(0, email);
            }
        }
        self.save_emails(&emails)?;
        Ok(())
    }

    pub fn update_email(&self, id: &str, email: Email) -> Result<()> {
        let mut emails = self.emails.lock().unwrap();
        if let Some(pos) = emails.iter().position(|e| e.id == id) {
            emails[pos] = email;
            self.save_emails(&emails)?;
        }
        Ok(())
    }

    pub fn delete_email(&self, id: &str) -> Result<()> {
        let mut emails = self.emails.lock().unwrap();
        emails.retain(|e| e.id != id);
        self.save_emails(&emails)?;
        Ok(())
    }

    pub fn get_settings(&self) -> Result<AppSettings> {
        let settings = self.settings.lock().unwrap();
        Ok(settings.clone())
    }

    pub fn update_settings(&self, new_settings: AppSettings) -> Result<()> {
        let mut settings = self.settings.lock().unwrap();
        *settings = new_settings;
        self.save_settings(&settings)?;
        Ok(())
    }

    fn save_accounts(&self, accounts: &[EmailAccount]) -> Result<()> {
        let path = self.data_dir.join("accounts.json");
        let data = serde_json::to_string_pretty(accounts)?;
        fs::write(path, data)?;
        Ok(())
    }

    fn save_emails(&self, emails: &[Email]) -> Result<()> {
        let path = self.data_dir.join("emails.json");
        let data = serde_json::to_string_pretty(emails)?;
        fs::write(path, data)?;
        Ok(())
    }

    fn save_settings(&self, settings: &AppSettings) -> Result<()> {
        let path = self.data_dir.join("settings.json");
        let data = serde_json::to_string_pretty(settings)?;
        fs::write(path, data)?;
        Ok(())
    }
}
