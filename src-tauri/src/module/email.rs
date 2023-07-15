use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailModel {
    pub from: String,
    pub to: String,
    pub subject: String,
    pub body: String,
}

impl EmailModel {
    pub fn new(from: String, to: String, subject: String, body: String) -> EmailModel {
        EmailModel {
            from,
            to,
            subject,
            body,
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
    println!("Fetching emails from {}...", domain);

    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, port), domain, &tls).unwrap();

    let mut imap_session = client.login(email, password).unwrap();

    imap_session.select(mailbox).unwrap();
    let messages = imap_session.fetch(sequence, "RFC822").unwrap();

    let mut emails: Vec<EmailModel> = Vec::new();

    for message in messages.iter() {
        let body = match message.body() {
            Some(body) => std::str::from_utf8(body).unwrap().to_string(),
            None => "".to_string(),
        };

        let subject = match message.envelope() {
            Some(envelope) => match envelope.subject {
                Some(subject) => std::str::from_utf8(subject).unwrap().to_string(),
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        let to = match message.envelope() {
            Some(envelope) => match &envelope.to {
                Some(to) => match to.get(0) {
                    Some(to) => match to.name {
                        Some(to) => std::str::from_utf8(to).unwrap().to_string(),
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                },
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        let from = match message.envelope() {
            Some(envelope) => match &envelope.from {
                Some(from) => match from.get(0) {
                    Some(from) => match from.name {
                        Some(from) => std::str::from_utf8(from).unwrap().to_string(),
                        None => "".to_string(),
                    },
                    None => "".to_string(),
                },
                None => "".to_string(),
            },
            None => "".to_string(),
        };

        emails.push(EmailModel::new(from, to, subject, body));
    }

    emails
}
