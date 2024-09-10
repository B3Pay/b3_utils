use crate::{
    memory::{
        init_stable_mem_refcell,
        types::{DefaultStableBTreeMap, DefaultStableCell, Storable},
    },
    HttpOutcall, HttpOutcallResponse,
};
use candid::CandidType;
use enum_dispatch::enum_dispatch;
use ic_stable_structures::storable::Bound;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

thread_local! {
    static CURRENT_ID_COUNTER: RefCell<DefaultStableCell<u64>> = init_stable_mem_refcell("counter_id", 220).unwrap();
    static EMAIL_PROVIDER_KEY_MAP: RefCell<DefaultStableBTreeMap<EmailProviderType, AppKeyToken>> = init_stable_mem_refcell("app_key", 221).unwrap();
}

fn get_app_key(provider: &EmailProviderType) -> AppKeyToken {
    EMAIL_PROVIDER_KEY_MAP.with(|c| {
        let map = c.borrow();
        map.get(provider).unwrap().clone()
    })
}

pub fn set_app_key(provider: EmailProviderType, key: String, url: String) {
    EMAIL_PROVIDER_KEY_MAP.with(|c| {
        let mut map = c.borrow_mut();
        map.insert(provider, AppKeyToken { key, url });
    });
}

fn get_next_id() -> u64 {
    let current_id: u64 = CURRENT_ID_COUNTER.with(|c| *c.borrow().get());

    CURRENT_ID_COUNTER.with(|c| {
        let mut counter = c.borrow_mut();
        counter.set(current_id + 1).unwrap();
    });

    current_id
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Serialize, Deserialize)]
struct AppKeyToken {
    key: String,
    url: String,
}

#[derive(CandidType, Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Serialize, Deserialize)]
pub enum EmailProviderType {
    Resend,
    Courier,
}

impl Storable for EmailProviderType {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let string = String::from_utf8_lossy(&bytes);
        match string.as_ref() {
            "Resend" => EmailProviderType::Resend,
            "Courier" => EmailProviderType::Courier,
            _ => panic!("Invalid EmailProviderType"),
        }
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        match self {
            EmailProviderType::Resend => "Resend".to_string().into_bytes().into(),
            EmailProviderType::Courier => "Courier".to_string().into_bytes().into(),
        }
    }
}

impl Storable for AppKeyToken {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let string = String::from_utf8_lossy(&bytes);
        let parts: Vec<&str> = string.split(',').collect();
        AppKeyToken {
            key: parts[0].to_string(),
            url: parts[1].to_string(),
        }
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        format!("{},{}", self.key, self.url).into_bytes().into()
    }
}

#[enum_dispatch]
pub trait EmailTrait {
    fn body(&self) -> Vec<u8>;
    fn url(&self) -> String;
    fn provider_type(&self) -> EmailProviderType;
}

#[enum_dispatch(EmailTrait)]
#[derive(Deserialize, CandidType, Clone)]
pub enum EmailProvider {
    Resend(Resend),
    Courier(Courier),
}

#[derive(Deserialize, CandidType, Clone)]
pub struct Resend {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub content: String,
}

impl EmailTrait for Resend {
    fn body(&self) -> Vec<u8> {
        serde_json::json!({
            "from": self.from,
            "to": [self.to],
            "subject": self.subject,
            "html": self.content,
        })
        .to_string()
        .into_bytes()
    }

    fn url(&self) -> String {
        get_app_key(&EmailProviderType::Resend).url
    }

    fn provider_type(&self) -> EmailProviderType {
        EmailProviderType::Resend
    }
}

#[derive(CandidType, Clone, Serialize, Deserialize)]
pub enum CourierProvider {
    Gmail,
    Resend,
    // Postmark,
    // SendGrid,
}

#[derive(Deserialize, CandidType, Clone)]
pub struct Courier {
    pub to: String,
    pub title: String,
    pub content: String,
    pub provider: CourierProvider,
}

impl EmailTrait for Courier {
    fn body(&self) -> Vec<u8> {
        serde_json::json!({
            "message": {
                "to": {
                    "email": self.to
                },
                "content": {
                    "title": self.title,
                    "body": self.content
                },
                "routing": {
                    "method": "single",
                    "channels": [match self.provider {
                        CourierProvider::Gmail => "gmail",
                        CourierProvider::Resend => "resend",
                        // CourierProvider::Postmark => "postmark",
                        // CourierProvider::SendGrid => "sendgrid",
                    }]
                }
            }
        })
        .to_string()
        .into_bytes()
    }

    fn url(&self) -> String {
        get_app_key(&EmailProviderType::Courier).url
    }

    fn provider_type(&self) -> EmailProviderType {
        EmailProviderType::Courier
    }
}

pub struct EmailBuilder {
    provider: EmailProviderType,
    to: Option<String>,
    from: Option<String>,
    subject: Option<String>,
    content: Option<String>,
    courier_provider: Option<CourierProvider>,
}

impl From<EmailProvider> for EmailBuilder {
    fn from(email: EmailProvider) -> Self {
        match email {
            EmailProvider::Resend(resend) => EmailBuilder {
                provider: EmailProviderType::Resend,
                to: Some(resend.to),
                from: Some(resend.from),
                subject: Some(resend.subject),
                content: Some(resend.content),
                courier_provider: None,
            },
            EmailProvider::Courier(courier) => EmailBuilder {
                provider: EmailProviderType::Courier,
                to: Some(courier.to),
                from: None,
                subject: Some(courier.title),
                content: Some(courier.content),
                courier_provider: Some(courier.provider),
            },
        }
    }
}

impl EmailBuilder {
    pub fn new(provider: EmailProviderType) -> Self {
        EmailBuilder {
            provider,
            to: None,
            from: None,
            subject: None,
            content: None,
            courier_provider: None,
        }
    }

    pub fn provider(mut self, provider: EmailProviderType) -> Self {
        self.provider = provider;
        self
    }

    pub fn courier_provider(mut self, provider: CourierProvider) -> Self {
        self.courier_provider = Some(provider);
        self
    }

    pub fn to(mut self, to: &str) -> Self {
        self.to = Some(to.to_string());
        self
    }

    pub fn from(mut self, from: &str) -> Self {
        self.from = Some(from.to_string());
        self
    }

    pub fn subject(mut self, subject: &str) -> Self {
        self.subject = Some(subject.to_string());
        self
    }

    pub fn content(mut self, content: &str) -> Self {
        self.content = Some(content.to_string());
        self
    }

    pub fn build(self) -> Result<EmailProvider, String> {
        match self.provider {
            EmailProviderType::Resend => {
                let resend = Resend {
                    to: self.to.ok_or("To address is required")?,
                    from: self.from.ok_or("From address is required")?,
                    subject: self.subject.ok_or("Subject is required")?,
                    content: self.content.ok_or("Content is required")?,
                };
                Ok(EmailProvider::Resend(resend))
            }
            EmailProviderType::Courier => {
                let courier = Courier {
                    to: self.to.ok_or("To address is required")?,
                    title: self.subject.ok_or("Subject is required")?,
                    content: self.content.ok_or("Content is required")?,
                    provider: self
                        .courier_provider
                        .ok_or("Courier provider is required")?,
                };
                Ok(EmailProvider::Courier(courier))
            }
        }
    }
}

pub async fn send_email(email: EmailProvider) -> Result<HttpOutcallResponse, String> {
    let app_key = get_app_key(&email.provider_type());
    let idempotency_key = get_next_id().to_string();

    let http_request = HttpOutcall::new(&email.url())
        .max_response_bytes(Some(1024))
        .post(&String::from_utf8_lossy(&email.body()), None)
        .add_headers(vec![
            ("User-Agent".to_string(), "Mozilla/5.0".to_string()),
            ("Content-Type".to_string(), "application/json".to_string()),
            (
                "Authorization".to_string(),
                format!("Bearer {}", app_key.key),
            ),
            ("Idempotency-Key".to_string(), idempotency_key),
        ]);

    http_request
        .send_with_closure(|raw| HttpOutcallResponse {
            status: raw.status,
            body: raw.body,
            headers: Vec::new(),
        })
        .await
}
