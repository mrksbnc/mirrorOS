#[tauri::command(async)]
pub fn get_unseen() -> crate::models::command_result::CommandResult<Vec<super::model::Email>> {
    let mut emails: Vec<super::model::Email> = Vec::new();
    let client_configs = super::config::get_imap_client_config();

    for client_config in client_configs {
        let instance = super::client::Imap::new(&client_config.host, client_config.port);

        let result = instance.fetch_unseen(
            &client_config.email,
            &client_config.password,
            &client_config.sequence,
            &client_config.mailbox,
        );

        match result.success {
            true => match result.data {
                Some(data) => {
                    for email in data {
                        emails.push(email);
                    }
                }
                None => {
                    return crate::models::command_result::CommandResult {
                        success: false,
                        data: None,
                        message: "No emails found".to_string(),
                    };
                }
            },
            false => {
                return crate::models::command_result::CommandResult {
                    success: false,
                    data: None,
                    message: result.message,
                };
            }
        }
    }

    return crate::models::command_result::CommandResult {
        success: true,
        data: Some(emails),
        message: "Emails fetched successfully".to_string(),
    };
}
