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
    static IDEMPOTENCY_KEY_COUNTER: RefCell<DefaultStableCell<u64>> = init_stable_mem_refcell("idempotency_key", 249).unwrap();
    static EMAIL_PROVIDER_KEY_MAP: RefCell<DefaultStableBTreeMap<EmailProviderType, AppKeyToken>> = init_stable_mem_refcell("provider_key_map", 248).unwrap();
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
    let current_id: u64 = IDEMPOTENCY_KEY_COUNTER.with(|c| *c.borrow().get());

    IDEMPOTENCY_KEY_COUNTER.with(|c| {
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
    OneSignal,
    CloudflareWorker, // Add the new provider type
}

impl Storable for EmailProviderType {
    const BOUND: Bound = Bound::Unbounded;

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        let string = String::from_utf8_lossy(&bytes);
        match string.as_ref() {
            "Resend" => EmailProviderType::Resend,
            "Courier" => EmailProviderType::Courier,
            "OneSignal" => EmailProviderType::OneSignal,
            "CloudflareWorker" => EmailProviderType::CloudflareWorker, // Add this line
            _ => panic!("Invalid EmailProviderType"),
        }
    }

    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        match self {
            EmailProviderType::Resend => "Resend".to_string().into_bytes().into(),
            EmailProviderType::Courier => "Courier".to_string().into_bytes().into(),
            EmailProviderType::OneSignal => "OneSignal".to_string().into_bytes().into(),
            EmailProviderType::CloudflareWorker => {
                "CloudflareWorker".to_string().into_bytes().into()
            } // Add this line
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
    OneSignal(OneSignal),
    CloudflareWorker(CloudflareWorker),
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
    OneSignal,
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
                        CourierProvider::OneSignal => "onesignal",
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

#[derive(Deserialize, CandidType, Clone)]
pub struct OneSignal {
    pub app_id: String,
    pub email_subject: String,
    pub email_body: String,
    pub include_email_tokens: Vec<String>,
}

impl EmailTrait for OneSignal {
    fn body(&self) -> Vec<u8> {
        serde_json::json!({
            "app_id": self.app_id,
            "email_subject": self.email_subject,
            "email_body": self.email_body,
            "include_email_tokens": self.include_email_tokens,
        })
        .to_string()
        .into_bytes()
    }

    fn url(&self) -> String {
        get_app_key(&EmailProviderType::OneSignal).url
    }

    fn provider_type(&self) -> EmailProviderType {
        EmailProviderType::OneSignal
    }
}
#[derive(Deserialize, CandidType, Clone)]
pub struct CloudflareWorker {
    pub to: String,
    pub from: String,
    pub subject: String,
    pub content: String,
}

impl EmailTrait for CloudflareWorker {
    fn body(&self) -> Vec<u8> {
        serde_json::json!({
            "to": self.to,
            "subject": self.subject,
            "html": self.content,
            "from": self.from,
        })
        .to_string()
        .into_bytes()
    }

    fn url(&self) -> String {
        get_app_key(&EmailProviderType::CloudflareWorker).url
    }

    fn provider_type(&self) -> EmailProviderType {
        EmailProviderType::CloudflareWorker
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

#[derive(CandidType, Debug, Serialize, Deserialize)]
pub struct SendEmailArgs {
    pub to: String,
    pub subject: String,
    pub message: String,
}
