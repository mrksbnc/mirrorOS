use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailModel {
    pub uuid: String,
    pub from: String,
    pub to: String,
    pub subject: String,
    pub created_at: String,
}

impl EmailModel {
    pub fn new(uuid: String, from: String, to: String, subject: String) -> EmailModel {
        EmailModel {
            uuid,
            from,
            to,
            subject,
            created_at: chrono::Local::now().to_string(),
        }
    }
}

pub fn fetch_emails(
    port: u16,
    domain: &str,
    email: &str,
    password: &str,
    sequence: &str,
    mailbox: &str,
) -> Vec<EmailModel> {
    println!("--------------------------------------------");
    println!("* Fetching emails from {} server...", domain);

    let mut emails: Vec<EmailModel> = Vec::new();
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, port), domain, &tls).unwrap();

    let mut imap_session = match client.login(email, password) {
        Ok(c) => c,
        Err(e) => {
            println!("* Error logging in to IMAP server: {:?}", e);
            return emails;
        }
    };

    imap_session.select(mailbox).unwrap();

    let messages = match imap_session.fetch(sequence, "(FLAGS ENVELOPE)") {
        Ok(m) => {
            match imap_session.run_command("LOGOUT") {
                Ok(_) => {
                    println!("* Logged out of {}...", domain);
                }
                Err(e) => {
                    println!("* Error logging out of {} server: {:?}", domain, e);
                }
            }
            m
        }
        Err(e) => {
            println!("* Error fetching emails: {:?}", e);
            return emails;
        }
    };

    let unseen_messages = messages
        .iter()
        .filter(|m| m.flags().iter().all(|f| f != &imap::types::Flag::Seen))
        .collect::<Vec<_>>();

    println!(
        "* Successfully fetched {} emails {} unread from {}...!",
        messages.len(),
        unseen_messages.len(),
        domain
    );

    for message in unseen_messages.iter() {
        if let Some(envelope) = message.envelope() {
            let uuid = match &envelope.message_id {
                Some(uuid) => std::str::from_utf8(uuid).unwrap().to_string(),
                None => "".to_string(),
            };

            let from = match &envelope.from {
                Some(from) => {
                    let mut sender_str = String::new();
                    for address in from {
                        let mailbox = match &address.mailbox {
                            Some(mailbox) => std::str::from_utf8(mailbox).unwrap().to_string(),
                            None => "".to_string(),
                        };

                        let host = match &address.host {
                            Some(host) => std::str::from_utf8(host).unwrap().to_string(),
                            None => "".to_string(),
                        };

                        let iterated_sender_str = format!("{}@{}", mailbox, host);

                        if sender_str.len() > 0 {
                            sender_str.push_str(", ");
                            sender_str.push_str(iterated_sender_str.as_str());
                        } else {
                            sender_str.push_str(iterated_sender_str.as_str());
                        }
                    }
                    sender_str
                }
                None => "".to_string(),
            };

            let to = match &envelope.to {
                Some(to) => {
                    let mut recipient_str = String::new();
                    for address in to {
                        let mailbox = match &address.mailbox {
                            Some(mailbox) => std::str::from_utf8(mailbox).unwrap().to_string(),
                            None => "".to_string(),
                        };

                        let host = match &address.host {
                            Some(host) => std::str::from_utf8(host).unwrap().to_string(),
                            None => "".to_string(),
                        };

                        let iterated_recipient_str = format!("{}@{}", mailbox, host);

                        if recipient_str.len() > 0 {
                            recipient_str.push_str(", ");
                            recipient_str.push_str(iterated_recipient_str.as_str());
                        } else {
                            recipient_str.push_str(iterated_recipient_str.as_str());
                        }
                    }
                    recipient_str
                }
                None => "".to_string(),
            };

            let subject = match &envelope.subject {
                Some(subject) => {
                    let subj = std::str::from_utf8(subject).unwrap().to_string();
                    let non_utf8 = String::from("=?UTF-8?");
                    let equals = String::from("=");
                    let tripple_slash = String::from("///");
                    let tripple_backslash = String::from("\\\\\\");
                    let clean = subj
                        .replace(&non_utf8, "")
                        .replace(&tripple_backslash, "")
                        .replace(&tripple_slash, "")
                        .replace(&equals, " ");

                    clean
                }
                None => "".to_string(),
            };

            let email = EmailModel::new(uuid, from, to, subject);

            emails.push(email);
        } else {
            println!("* Message didn't have an envelope!");
        }
    }

    emails
}
