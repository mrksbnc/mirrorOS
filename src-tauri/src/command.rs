use crate::module::{self};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CommandResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

impl<T> CommandResult<T> {
    pub fn new(success: bool, data: Option<T>, message: String) -> CommandResult<T> {
        CommandResult {
            success,
            data,
            message,
        }
    }
}

#[tauri::command()]
pub fn get_emails(
    port: u16,
    domain: &str,
    email: &str,
    password: &str,
    sequence: &str,
    mailbox: &str,
) -> CommandResult<Vec<module::email::EmailModel>> {
    let emails = module::email::fetch_emails(port, domain, email, password, sequence, mailbox);

    let command_result: CommandResult<Vec<module::email::EmailModel>> = CommandResult::new(
        true,
        Some(emails),
        "Emails fetched successfully".to_string(),
    );

    println!(
        "get_emails executed successfully! result: {:?}",
        command_result
    );

    command_result
}
