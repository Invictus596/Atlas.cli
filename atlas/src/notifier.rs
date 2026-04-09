use reqwest::Client;
use serde::Serialize;

/// Notifier module for Project Atlas.
/// Handles sending notifications via Twilio (SMS/WhatsApp).

#[derive(Serialize)]
struct TwilioMessageRequest {
    #[serde(rename = "To")]
    to: String,
    #[serde(rename = "From")]
    from: String,
    #[serde(rename = "Body")]
    body: String,
}

/// Send an SMS via Twilio.
pub async fn send_sms(
    sid: &str,
    token: &str,
    from: &str,
    to: &str,
    body: &str,
) -> Result<String, String> {
    let client = Client::new();
    let url = format!("https://api.twilio.com/2010-04-01/Accounts/{}/Messages.json", sid);

    let response = client
        .post(&url)
        .basic_auth(sid, Some(token))
        .form(&[("To", to), ("From", from), ("Body", body)])
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(format!("Twilio error ({}): {}", status, body));
    }

    Ok("Notification sent successfully".to_string())
}

/// Send a WhatsApp message via Twilio.
pub async fn send_whatsapp(
    sid: &str,
    token: &str,
    from: &str,
    to: &str,
    body: &str,
) -> Result<String, String> {
    let from_whatsapp = format!("whatsapp:{}", from);
    let to_whatsapp = format!("whatsapp:{}", to);
    send_sms(sid, token, &from_whatsapp, &to_whatsapp, body).await
}
