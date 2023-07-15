use crate::globals;
use crate::modules;
use crate::modules::email::Email;

#[tauri::command(async)]
pub fn fetch_emails(
    port: u16,
    domain: &str,
    email: &str,
    password: &str,
) -> globals::TResult<Vec<Email>> {
    let emails = modules::email::fetch_emails(port, domain, email, password);

    globals::TResult::new(
        true,
        Some(emails),
        "Emails fetched successfully".to_string(),
    )
}
