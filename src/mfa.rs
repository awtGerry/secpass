/* multi-factor authentication */

use serde::{Deserialize, Serialize};
use crate::passwd::User;

/* Struct to hold the user (only needed) */
#[derive(Deserialize)]
pub struct VerifyCode {
    // pub phone: String,
    pub user: User,
    pub code: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyCodeResponse {
    pub status: String
}

pub struct TwilioService {
    service_sid: String,
    ssid: String,
    auth_token: String,
}

impl TwilioService {
    pub fn new() -> TwilioService {
        TwilioService {
            service_sid: std::env::var("TWILIO_SERVICE_SID").unwrap(),
            ssid: std::env::var("TWILIO_ACCOUNT_SID").unwrap(),
            auth_token: std::env::var("TWILIO_AUTH_TOKEN").unwrap(),
        }
    }

    pub async fn send_otp(&self, phone: String) -> Result<(), &'static str> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://verify.twilio.com/v2/Services/{}/Verifications",
            self.service_id
        );

        let body = format!(
            r#"{{"To":"{}","Channel":"sms"}}"#,
            phone
        );

        let res = client
            .post(&url)
            .basic_auth(&self.ssid, Some(&self.auth_token))
            .header("Content-Type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();

        if res.status().is_success() {
            Ok(())
        } else {
            Err("Failed to send OTP")
        }
    }
}

impl VerifyCode {
    pub fn new(usr: User) -> VerifyCode {
        VerifyCode {
            user: usr,
            code: "".to_string(),
        }
    }
}
