/* multi-factor authentication */

<<<<<<< Updated upstream
use dotenv::dotenv;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
=======
use serde::{Deserialize, Serialize};
use crate::passwd::User;
use dotenv::dotenv;
>>>>>>> Stashed changes

pub struct Mfa {
    pub email: String, // To Email
    pub code: u16, // The code
    key: String, // smtp_key: defined in the .env file
}

const EMAIL: &str = "758071001@smtp-brevo.com";
const HOST: &str = "smtp-relay.sendinblue.com";

<<<<<<< Updated upstream
impl Mfa {
    pub fn new(email: String) -> Self {
        dotenv().ok();
        // Generate a random 4 digit code
        let mfa_code = rand::random::<u16>() % 10000;
        let smtp_key = std::env::var("SMTP_KEY").expect("SMTP_KEY must be set");
        println!("Email: {}", email);
        println!("MFA code: {}", mfa_code);
        println!("SMTP key: {}", smtp_key);
        Self {
            email,
            code: mfa_code,
            key: smtp_key,
=======
pub struct TwilioService {
    service_sid: String,
    ssid: String,
    auth_token: String,
}

impl TwilioService {
    pub fn new() -> TwilioService {
        dotenv().ok();
        TwilioService {
            service_sid: std::env::var("TWILIO_SERVICE_SID").expect("TWILIO_SERVICE_SID not found"),
            ssid: std::env::var("TWILIO_ACCOUNT_SID").expect("TWILIO_ACCOUNT_SID not found"),
            auth_token: std::env::var("TWILIO_AUTH_TOKEN").expect("TWILIO_AUTH_TOKEN not found"),
>>>>>>> Stashed changes
        }
    }

    pub fn send(&self) {
        let to_email = self.email.clone();
        let key = self.key.clone();
        let email = Message::builder()
            .from(EMAIL.parse().unwrap())
            .to(to_email.parse().unwrap())
            .subject("Secpass verification code")
            .body(format!("Your verification code is: {}", self.code))
            .unwrap();

        let creds = Credentials::new(EMAIL.to_string(), key);
        let mailer = SmtpTransport::relay(HOST)
            .unwrap()
            .credentials(creds)
            .build();

        match mailer.send(&email) {
            Ok(_) => println!("Email sent successfully"),
            Err(e) => println!("Could not send email: {:?}", e),
        }
    }
}
