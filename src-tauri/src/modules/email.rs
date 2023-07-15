#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Email {
    pub to: String,
    pub from: String,
    pub body: String,
    pub subject: String,
}

impl Email {
    pub fn new(to: String, from: String, body: String, subject: String) -> Email {
        Email {
            to,
            from,
            body,
            subject,
        }
    }
}

pub fn fetch_emails(port: u16, domain: &str, email: &str, password: &str) -> Vec<Email> {
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, port), domain, &tls).unwrap();

    let mut imap_session = client.login(email, password).unwrap();

    imap_session.select("INBOX").unwrap();
    let messages = imap_session.fetch("1:3", "RFC822").unwrap();

    let mut emails: Vec<Email> = Vec::new();

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

        let email = Email::new(to, from, body, subject);
        emails.push(email);
    }

    emails
}
