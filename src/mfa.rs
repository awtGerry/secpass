/* multi-factor authentication */

use serde::{Deserialize, Serialize};
use crate::passwd::User;
use dotenv::dotenv;

pub struct Mfa {
    pub email: String, // To Email
    pub code: u16, // The code
    pub msg: String, // The message if any
    key: String, // smtp_key: defined in the .env file
}

const EMAIL: &str = "758071001@smtp-brevo.com";
const HOST: &str = "smtp-relay.sendinblue.com";

impl Mfa {
    pub fn new(email: String) -> Self {
        dotenv().ok();
        // Generate a random 6-digit code
        let mfa_code: u16 = rand::random();
        let smtp_key = std::env::var("SMTP_KEY").expect("SMTP_KEY must be set");
        println!("Email: {}", email);
        println!("MFA code: {}", mfa_code);
        println!("SMTP key: {}", smtp_key);
        Self {
            email,
            code: mfa_code,
            msg: String::new(),
            key: smtp_key,
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
