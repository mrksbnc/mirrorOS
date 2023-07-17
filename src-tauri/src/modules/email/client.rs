use crate::models::command_result::CommandResult;

use super::model::Email;

use native_tls::TlsStream;
use std::net::TcpStream;

#[derive(Debug)]
pub struct Imap {
    pub client: imap::Client<TlsStream<TcpStream>>,
}

impl Imap {
    pub fn new(domain: &str, port: u16) -> Self {
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect((domain, port), domain, &tls).unwrap();

        Imap { client }
    }

    pub fn fetch_unseen(
        self,
        email: &str,
        password: &str,
        sequence: &str,
        mailbox: &str,
    ) -> CommandResult<Vec<Email>> {
        let mut emails: Vec<Email> = Vec::new();

        let mut imap_session = match self.client.login(email, password) {
            Ok(c) => c,
            Err(e) => {
                log::error!("Error logging in to IMAP server: {:?}", e);

                return CommandResult::new(
                    false,
                    None,
                    "Error logging in to IMAP server".to_string(),
                );
            }
        };

        imap_session.select(mailbox).unwrap();

        let uuid_collection = match imap_session.uid_fetch(sequence, "(FLAGS ENVELOPE)") {
            Ok(m) => m,
            Err(e) => {
                log::error!("Error fetching emails: {:?}", e);

                return CommandResult::new(false, None, "Error fetching emails".to_string());
            }
        };

        let unseen_uuid_collection = uuid_collection
            .iter()
            .filter(|m| m.flags().iter().all(|f| f != &imap::types::Flag::Seen))
            .collect::<Vec<_>>();

        if unseen_uuid_collection.len() == 0 {
            log::info!("No unseen messages were found");

            return CommandResult::new(true, None, "No unseen messages were found".to_string());
        }

        let uuid_map = unseen_uuid_collection
            .iter()
            .map(|m| m.uid)
            .collect::<Vec<_>>();

        let mut uid_set = String::new();

        for uid in uuid_map.iter() {
            if uid_set.len() > 0 {
                uid_set.push_str(",");
                uid_set.push_str(uid.unwrap().to_string().as_str());
            } else {
                uid_set.push_str(uid.unwrap().to_string().as_str());
            }
        }

        log::info!(
            "{} unseen messages were found out of {} total messages",
            uuid_collection.len(),
            unseen_uuid_collection.len(),
        );

        log::info!("uuid set {}", uid_set.len());

        let messages = match imap_session.uid_fetch(uid_set, "(FLAGS ENVELOPE)") {
            Ok(m) => m,
            Err(e) => {
                log::error!("Error fetching emails: {:?}", e);

                return CommandResult::new(false, None, "Error fetching emails".to_string());
            }
        };

        let unseen_messages = messages
            .iter()
            .filter(|m| m.flags().iter().all(|f| f != &imap::types::Flag::Seen))
            .collect::<Vec<_>>();

        log::info!(
            "Successfully fetched {} emails {} unread from IMAP server",
            messages.len(),
            unseen_messages.len(),
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

                let email = Email::new(uuid, from, to, subject);

                emails.push(email);
            } else {
                log::info!("Message didn't have an envelope!");
            }
        }

        CommandResult::new(
            true,
            Some(emails),
            "Successfully fetched emails".to_string(),
        )
    }
}
