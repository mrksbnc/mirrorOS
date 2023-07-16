use crate::{
    models::{command::CommandResult, email::Email},
    modules,
};

#[tauri::command(async)]
pub fn get_unseen(
    port: u16,
    domain: &str,
    email: &str,
    password: &str,
    sequence: &str,
    mailbox: &str,
) -> CommandResult<Vec<Email>> {
    return modules::email::client::ImapClient::new(domain, port)
        .fetch_unseen(email, password, sequence, mailbox);
}
