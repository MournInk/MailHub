mod types;
mod storage;
mod email;
mod ai;

use std::sync::Arc;
use tauri::{AppHandle, Manager, State};
use types::*;
use storage::Store;

struct AppState {
    store: Arc<Store>,
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! Welcome to MailHub!", name)
}

#[tauri::command]
async fn get_accounts(state: State<'_, AppState>) -> Result<Vec<EmailAccount>, String> {
    state.store.get_accounts().map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_account(
    account: EmailAccount,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut new_account = account;
    if new_account.id.is_empty() {
        new_account.id = uuid::Uuid::new_v4().to_string();
    }
    state.store.add_account(new_account).map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_account(
    id: String,
    account: EmailAccount,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.store.update_account(&id, account).map_err(|e| e.to_string())
}

#[tauri::command]
async fn delete_account(
    id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.store.delete_account(&id).map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_emails(state: State<'_, AppState>) -> Result<Vec<Email>, String> {
    state.store.get_emails().map_err(|e| e.to_string())
}

#[tauri::command]
async fn sync_emails(
    state: State<'_, AppState>,
    app: AppHandle,
) -> Result<(), String> {
    let accounts = state.store.get_accounts().map_err(|e| e.to_string())?;
    let settings = state.store.get_settings().map_err(|e| e.to_string())?;
    
    for account in accounts {
        match email::EmailClient::fetch_emails(&account).await {
            Ok(mut emails) => {
                // Classify emails with AI if enabled
                if let Some(ai_config) = &settings.ai_config {
                    if ai_config.enabled {
                        let classifier = ai::AIClassifier::new(ai_config.clone());
                        for email_item in &mut emails {
                            if let Ok(classification) = classifier.classify_email(email_item).await {
                                email_item.ai_classification = Some(classification.clone());
                                
                                // Auto-delete marketing emails if configured
                                if ai_config.auto_delete && matches!(classification.category, Category::Marketing) {
                                    continue; // Skip adding this email
                                }
                                
                                // Send notification for important emails
                                if classification.should_notify && settings.notifications {
                                    // Note: Notifications would be sent here using tauri-plugin-notification
                                    // For now, just log it
                                    println!("Notification: {} - From: {}", &email_item.subject, &email_item.from.address);
                                }
                            }
                        }
                    }
                }
                
                state.store.add_emails(emails).map_err(|e| e.to_string())?;
            }
            Err(e) => eprintln!("Failed to fetch emails for account {}: {}", account.id, e),
        }
    }
    
    Ok(())
}

#[tauri::command]
async fn send_email(
    from_account_id: String,
    to: String,
    subject: String,
    body: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let accounts = state.store.get_accounts().map_err(|e| e.to_string())?;
    let account = accounts.iter()
        .find(|a| a.id == from_account_id)
        .ok_or_else(|| "Account not found".to_string())?;
    
    email::send_email(account, &to, &subject, &body)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    state.store.get_settings().map_err(|e| e.to_string())
}

#[tauri::command]
async fn update_settings(
    settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    state.store.update_settings(settings).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_dir = app.path().app_data_dir()
                .expect("Failed to get app data directory");
            
            let store = Arc::new(
                Store::new(app_dir).expect("Failed to initialize store")
            );
            
            app.manage(AppState { store });
            
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_accounts,
            add_account,
            update_account,
            delete_account,
            get_emails,
            sync_emails,
            send_email,
            get_settings,
            update_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
