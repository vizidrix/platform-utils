use core::fmt::Formatter;
use reqwest::{ Client, Response };
use reqwest::header::{self, HeaderValue, CONTENT_TYPE, USER_AGENT};
use serde::{Serialize, Deserialize};
use std::fmt::Debug;

const APPLICTION_JSON: &str = "application/json";
const LIB_USER_AGENT: &str = concat!["CF-MAILCHANNELS", "/", env!("CARGO_PKG_VERSION")];
const MAILCHANNELS_SEND_API: &str = "https://api.mailchannels.net/tx/v1/send";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Participants(Vec<Participant>);

impl Participants {
    pub fn inner(&self) -> Vec<Participant> {
        self.0.clone()
    }
}

impl From<&str> for Participants {
    fn from(src: &str) -> Self {
        Participants(vec![src.into()])
    }
}

impl<P: Clone + Into<Participant>> From<Vec<P>> for Participants {
    fn from(src: Vec<P>) -> Self {
        Participants(src.iter().map(|p| p.clone().into()).collect())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Participant {
    pub email: String,
    pub name: String,
}

impl From<&str> for Participant {
    fn from(src: &str) -> Self {
        Participant {
            email: src.to_owned(),
            name: "".to_owned()
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Personalization {
    pub to: Vec<Participant>,
    pub dkim_domain: Option<String>,
    pub dkim_selector: Option<String>,
    pub dkim_private_key: Option<String>,
}

impl Debug for Personalization {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("Personalization")
            .field("to", &self.to)
            .finish()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Headers {
    #[serde(rename = "Date")]
    pub date: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Content {
    #[serde(rename = "type")]
    pub content_type: String,
    pub value: String,
}

impl From<&str> for Content {
    fn from(src: &str) -> Self {
        Content {
            content_type: "text/plain".to_owned(),
            value: src.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct EmailMessage {
    pub personalizations: Vec<Personalization>,
    pub from: Participant,
    pub headers: Option<Headers>,
    pub subject: String,
    pub content: Vec<Content>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct Dkim {
    pub domain: String,
    pub selector: String,
    pub private_key: String,
}

impl Debug for Dkim {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_struct("Dkim")
            .field("domain", &self.domain)
            .field("selector", &self.selector)
            .field("private_key", &"{hidden}")
            .finish()
    }
}

impl Dkim {
    pub fn new(domain: impl Into<String>, selector: impl Into<String>, private_key: impl Into<String>) -> Self {
        Dkim {
            domain: domain.into(),
            selector: selector.into(),
            private_key: private_key.into(),
        }
    }
}

impl EmailMessage {
    pub fn new(from: impl Into<Participant>, to: impl Into<Participants>, subject: impl Into<String>, content: impl Into<Content>) -> Self {
        Self::new_with_dkim(None, from, to, subject, content)
    }

    pub fn new_with_dkim(dkim: Option<Dkim>, from: impl Into<Participant>, to: impl Into<Participants>, subject: impl Into<String>, content: impl Into<Content>) -> Self {
        let dkim = dkim.as_ref();
        let from = from.into();
        let to = to.into();
        let subject = subject.into();
        let content = content.into();
        EmailMessage {
            personalizations: vec![ Personalization {
                to: to.0,
                dkim_domain: dkim.map(|v| v.domain.to_owned()),
                dkim_selector: dkim.map(|v| v.selector.to_owned()),
                dkim_private_key: dkim.map(|v| v.private_key.to_owned()),
            }],
            from,
            headers: None,
            subject,
            content: vec![content],
        }
    }
}

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
}

impl From<reqwest::Error> for Error {
    fn from(src: reqwest::Error) -> Self {
        Error::Reqwest(src)
    }
}

pub struct MailChannelsClient {
    client: Client,
}

impl Default for MailChannelsClient {
    fn default() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static(APPLICTION_JSON));
        headers.insert(USER_AGENT, HeaderValue::from_static(LIB_USER_AGENT));
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Reqwest client builder should not fail");
        MailChannelsClient { client }
    }
}

impl MailChannelsClient {
    pub async fn send(&self, email: EmailMessage) -> Result<Response, Error> {
        let response = self.client
            .post(MAILCHANNELS_SEND_API)
            .json(&email)
            .send()
            .await?;
        Ok(response)
    }
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn convert_string_to_participant() {
        let email = "me@acme.com".to_owned();
        let expected = Participant {
            email,
            name: String::new(),
        };
        let participant: Participant = "me@acme.com".into();
        assert_eq!(expected, participant);
    }

    #[test]
    fn convert_vec_string_to_participant() {
        let emails = vec!["me@acme.com", "you@acme.com"];
        let expected = vec![
            Participant {
                email: emails[0].to_owned(),
                name: String::new(),
            },
            Participant {
                email: emails[1].to_owned(),
                name: String::new(),
            },
        ];
        let participant: Participants = emails.into();
        assert_eq!(Participants(expected), participant);
    }

    #[test]
    fn serialize_message_as_json_and_back() {
        let email = EmailMessage::new("from@acme.com", "to@acme.com", "subject", "content");
        let json = serde_json::to_string(&email).unwrap();
        let deserialized = serde_json::from_str(&json).unwrap();
        assert_eq!(email, deserialized);
    }
}

// pub async fn send_email(email_message: EmailMessage) -> Result<Response, Error> {
//     let mut headers = header::HeaderMap::new();
//     headers.insert(CONTENT_TYPE, HeaderValue::from_static(APPLICTION_JSON));
//     headers.insert(USER_AGENT, HeaderValue::from_static(LIB_USER_AGENT));
//     let client = reqwest::Client::builder()
//         .default_headers(headers)
//         .build()?;
//     let response = client.post(MAILCHANNELS_SEND_API).json(&email_message).send().await?;
//     Ok(response)
// }

// pub async fn send_email(email_message: EmailMessage) -> Result<Response> {
//     let mut headers = header::HeaderMap::new();
//     headers.insert(CONTENT_TYPE, HeaderValue::from_static(APPLICTION_JSON));
//     headers.insert(USER_AGENT, HeaderValue::from_static(LIB_USER_AGENT));
//     let client_result = reqwest::Client::builder()
//         .default_headers(headers)
//         .build()?;
//     let client = match client_result {
//         Ok(client) => client,
//         Err(error) => return Response::error(format!("Client Error {error:?}"), 500),
//     };
//     // let payload = sample_email(dkim_private_key, message);
//     let result = client.post(MAILCHANNELS_SEND_API).json(&email_message).send().await;
//     let response = match result {
//         Ok(response) => response,
//         Err(error) => return Response::error(format!("Failed to send email {error:?}"), 500),
//     };
//     let status = response.status().as_u16();
//     if (200..300).contains(&status) {
//         return Response::ok(format!("{response:?}"));
//     }
//     let body = response.text().await.unwrap_or_default();
//     Response::error(format!("Email Error {body:?}"), 500)
// }

// fn sample_email(dkim_private_key: String, message: &str) -> EmailMessage {
//     EmailMessage {
//         personalizations: vec![
//             Personalization {
//                 to: vec![
//                     Participant {
//                         email: "userd@gmail.com".to_owned(),
//                         name: "John Doe".to_owned(),
//                     },
//                 ],
//                 dkim_domain: "domain.com".to_owned(),
//                 dkim_selector: "mailchannels".to_owned(),
//                 dkim_private_key,
//             },
//         ],
//         from: Participant {
//             email: "userd@domain.com".to_owned(),
//             name: "John Doe".to_owned(),
//         },
//         subject: "Some Stuff Is Cool".to_owned(),
//         content: vec![
//             Content {
//                 content_type: "text/plain".to_owned(),
//                 value: format!("Test message content\n\n{message:?}"),
//             }
//         ]
//     }
// }
