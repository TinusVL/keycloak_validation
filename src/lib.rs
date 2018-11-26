extern crate reqwest;

pub const AUTHORIZATION: &str = "Authorization";

pub fn verify(authorization_header: &str, host_url: &str, realm: &str) -> Result<String, String> {
    // Skip any prefix like 'Bearer ' of 'Authorization: Bearer '
    let access_token = authorization_header
        .split(' ')
        .last()
        .ok_or("Empty authorization header")?;

    let user_info_url = format!(
        "{}/auth/realms/{}/protocol/openid-connect/userinfo",
        host_url, realm
    );
    let mut response = reqwest::Client::new()
        .get(&user_info_url)
        .bearer_auth(access_token)
        .send()
        .map_err(|e| format!("{}", e))?;

    response.text().map_err(|e| format!("{}", e))
}
