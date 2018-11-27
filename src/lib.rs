#![warn(missing_docs)]
//! This crate allows user authentication, using the validation of a keycloak access token.
//! Local validation of a token is hard to get right, so the validation is delegated to the keycloak server.

extern crate reqwest;
extern crate serde_json;

/// This signifies that the user described in 'user_info' has been authenticated.
#[derive(Debug)]
pub struct Authentication {
    /// User info given during authentication
    pub user_info: serde_json::Value,
}

/// Verify an access token in an authorization header, using a keycloak server.
/// If this verification succeeds, then the user is authenicated.
pub fn verify(
    authorization_header: &str,
    host_url: &str,
    realm: &str,
    timeout: std::time::Duration,
) -> Result<Authentication, String> {
    // Skip any prefix like 'Bearer ' of 'Authorization: Bearer '
    let access_token = authorization_header
        .split(' ')
        .last()
        .ok_or("Empty authorization header")?;

    let realm_url = format!("{}/auth/realms/{}", host_url, realm);

    get_user_info(access_token, &realm_url, timeout)
        .map(|user_info| Authentication { user_info })
        .map_err(|e| format!("Authentication failure: {}", e))
}

/// Fetch user info from a keycloak server using the user's access token.
fn get_user_info(
    access_token: &str,
    realm_url: &str,
    timeout: std::time::Duration,
) -> Result<serde_json::Value, String> {
    let user_info_url = format!("{}/protocol/openid-connect/userinfo", realm_url);
    let mut response = reqwest::Client::builder()
        .timeout(timeout)
        .build()
        .map_err(|e| format!("Failed to build reqwest client: {}", e))?
        .get(&user_info_url)
        .bearer_auth(access_token)
        .send()
        .map_err(|e| format!("Sending failure: {}", e))?;

    if response.status().is_success() {
        response.json().map_err(|e| format!("JSON failure: {}", e))
    } else {
        Err(String::from(format!(
            "Invalid token - Status: {} - Body: {}",
            response.status(),
            response.text().unwrap_or(String::from("No text body"))
        )))
    }
}
