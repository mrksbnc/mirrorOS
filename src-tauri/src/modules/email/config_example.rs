pub fn get_imap_client_config() -> Vec<crate::models::config::ImapConfig> {
    let client_configs = vec![
        crate::models::config::ImapConfig::new(
            993,
            "outlook",
            "outlook.office365.com",
            "name@outlook.com",
            "app-password",
            "INBOX",
            "1:5",
        ),
        crate::models::config::ImapConfig::new(
            993,
            "iCloud",
            "imap.mail.me.com",
            "name@icloud.com",
            "app-password",
            "INBOX",
            "1:5",
        ),
    ];

    client_configs
}
