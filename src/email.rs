use crate::config::settings::EmailSettings;
use anyhow::{Context, Result};
use chrono;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::client::{Tls, TlsParameters};
use lettre::{Message, SmtpTransport, Transport};
use secrecy::ExposeSecret;
use tracing::{debug, error, info, warn};

#[derive(Clone)]
pub struct EmailService {
    settings: EmailSettings,
    mailer: Option<SmtpTransport>,
}

impl EmailService {
    pub fn new(settings: EmailSettings) -> Result<Self> {
        let mailer = if settings.test_mode {
            warn!("Email service running in TEST MODE - emails will not be sent via SMTP");
            None
        } else {
            let creds = Credentials::new(
                settings.smtp_username.clone(),
                settings.smtp_password.expose_secret().clone(),
            );

            let tls_parameters = TlsParameters::builder(settings.smtp_host.clone())
                .build()
                .context("Failed to build TLS parameters")?;
                
            info!("Configuring SMTP transport for {}:{} with STARTTLS", 
                  settings.smtp_host, settings.smtp_port);
            debug!("Using username: {}", settings.smtp_username);
                
            let mailer = SmtpTransport::relay(&settings.smtp_host)?
                .port(settings.smtp_port)
                .credentials(creds)
                .tls(Tls::Required(tls_parameters))
                .build();

            Some(mailer)
        };

        Ok(Self { settings, mailer })
    }

    pub async fn send_forgot_password_email(
        &self,
        to_email: &str,
        to_name: &str,
        reset_token: &str,
        reset_url: &str,
    ) -> Result<()> {
        let subject = "Reset Your Password - Aerugo ";
        let html_body = self.generate_forgot_password_html(to_name, reset_token, reset_url);
        let text_body = self.generate_forgot_password_text(to_name, reset_token, reset_url);

        self.send_email(to_email, to_name, subject, &html_body, &text_body)
            .await
    }

    async fn send_email(
        &self,
        to_email: &str,
        to_name: &str,
        subject: &str,
        html_body: &str,
        text_body: &str,
    ) -> Result<()> {
        if self.settings.test_mode {
            return self.save_test_email(to_email, subject, html_body).await;
        }

        let email = Message::builder()
            .from(
                format!("{} <{}>", self.settings.from_name, self.settings.from_email)
                    .parse()
                    .context("Failed to parse from email")?,
            )
            .to(format!("{} <{}>", to_name, to_email)
                .parse()
                .context("Failed to parse to email")?)
            .subject(subject)
            .multipart(
                lettre::message::MultiPart::alternative()
                    .singlepart(
                        lettre::message::SinglePart::builder()
                            .header(ContentType::TEXT_PLAIN)
                            .body(text_body.to_string()),
                    )
                    .singlepart(
                        lettre::message::SinglePart::builder()
                            .header(ContentType::TEXT_HTML)
                            .body(html_body.to_string()),
                    ),
            )
            .context("Failed to build email message")?;

        if let Some(ref mailer) = self.mailer {
            debug!("Attempting to send email to {} via SMTP", to_email);
            match mailer.send(&email) {
                Ok(response) => {
                    info!("Email sent successfully to {}: {:?}", to_email, response);
                    Ok(())
                }
                Err(e) => {
                    error!("Failed to send email to {}: {}", to_email, e);
                    debug!("SMTP error details: {:?}", e);
                    Err(anyhow::anyhow!("Failed to send email: {}", e))
                }
            }
        } else {
            error!("SMTP mailer not configured");
            Err(anyhow::anyhow!("SMTP mailer not configured"))
        }
    }

    async fn save_test_email(
        &self,
        to_email: &str,
        subject: &str,
        html_body: &str,
    ) -> Result<()> {
        let test_content = format!(
            "=== TEST EMAIL ===\n\
            To: {}\n\
            Subject: {}\n\
            Date: {}\n\
            \n\
            HTML Body:\n\
            {}\n\
            ==================\n\n",
            to_email,
            subject,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            html_body
        );

        let default_file = "test_emails.log".to_string();
        let file_path = self
            .settings
            .test_email_file
            .as_ref()
            .unwrap_or(&default_file);

        tokio::fs::write(file_path, test_content)
            .await
            .context("Failed to write test email to file")?;

        info!("Test email saved to file: {}", file_path);
        Ok(())
    }

    fn generate_forgot_password_html(
        &self,
        to_name: &str,
        reset_token: &str,
        reset_url: &str,
    ) -> String {
        format!(
            r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>Reset Your Password</title>
    <style>
        body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; max-width: 600px; margin: 0 auto; padding: 20px; }}
        .container {{ background: #f9f9f9; padding: 30px; border-radius: 10px; }}
        .header {{ background: #007bff; color: white; padding: 20px; text-align: center; border-radius: 5px; margin-bottom: 30px; }}
        .button {{ display: inline-block; background: #28a745; color: white; padding: 12px 30px; text-decoration: none; border-radius: 5px; font-weight: bold; margin: 20px 0; }}
        .button:hover {{ background: #218838; }}
        .token-box {{ background: #e9ecef; padding: 15px; border-radius: 5px; font-family: monospace; word-break: break-all; margin: 20px 0; }}
        .footer {{ color: #666; font-size: 12px; margin-top: 30px; text-align: center; }}
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>🔐 Aerugo</h1>
            <p>Password Reset Request</p>
        </div>
        
        <h2>Hello {}!</h2>
        
        <p>We received a request to reset your password for your Aerugo account.</p>
        
        <p><strong>Your password reset verification code is:</strong></p>
        
        <div style="text-align: center; margin: 30px 0;">
            <div style="display: inline-block; background: #007bff; color: white; padding: 20px 30px; border-radius: 10px; font-size: 32px; font-weight: bold; letter-spacing: 8px; font-family: monospace;">
                {}
            </div>
        </div>
        
        <p>Please enter this 6-digit code in the password reset form to continue.</p>
        
        <p><strong>Important:</strong></p>
        <ul>
            <li>This verification code will expire in 15 minutes</li>
            <li>If you didn't request this, you can safely ignore this email</li>
            <li>For security reasons, never share this code with anyone</li>
        </ul>
        
        <div class="footer">
            <p>© 2025 Aerugo  - Decenter.ai</p>
            <p>This email was sent from an automated system. Please do not reply.</p>
        </div>
    </div>
</body>
</html>"#,
            to_name, reset_token
        )
    }

    fn generate_forgot_password_text(
        &self,
        to_name: &str,
        reset_token: &str,
        _reset_url: &str,
    ) -> String {
        format!(
            r#"Hello {}!

We received a request to reset your password for your Aerugo  account.

Your password reset verification code is:

    {}

Please enter this 6-digit code in the password reset form to continue.

IMPORTANT:
- This verification code will expire in 15 minutes
- If you didn't request this, you can safely ignore this email  
- For security reasons, never share this code with anyone

© 2025 Aerugo  - Decenter.ai
This email was sent from an automated system. Please do not reply."#,
            to_name, reset_token
        )
    }
}