use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Email {
    pub uuid: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub created_at: String,
}

impl Email {
    pub fn new(uuid: String, from: String, to: String, subject: String) -> Email {
        Email {
            uuid,
            from,
            to,
            subject,
            created_at: chrono::Local::now().to_string(),
        }
    }
}
