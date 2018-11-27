extern crate keycloak_validation;

pub fn setup(port: &str, timout_millis: u64) {
    let port_serve = String::from(port);
    std::thread::spawn(move || {
        let keycloak_dummy_server = simple_server::Server::new(move |request, mut response| {
            std::thread::sleep(std::time::Duration::from_millis(timout_millis));

            if request.uri() == "/auth/realms/my-realm/protocol/openid-connect/userinfo" {
                let auth_header = request
                    .headers()
                    .get("Authorization")
                    .map(|h| h.to_str().unwrap_or("Failed to convert header"))
                    .unwrap_or("Missing header");
                if auth_header == "Bearer 012345678901234567890123456789" {
                    response.header("Content-Type", "application/json".as_bytes());
                    Ok(response.body("{\"user\":\"authenticated\"}".as_bytes().to_vec()).unwrap())
                } else {
                    response.status(simple_server::StatusCode::UNAUTHORIZED);
                    Ok(response
                        .body(format!("Wrong header: {}", auth_header).as_bytes().to_vec()).unwrap())
                }
            } else {
                response.status(simple_server::StatusCode::NOT_FOUND);
                Ok(response.body(format!("Wrong URL: {}", request.uri()).as_bytes().to_vec()).unwrap())
            }
        });

        keycloak_dummy_server.listen("localhost", &port_serve);
    });

    //std::thread::sleep(std::time::Duration::from_millis(200))
}

#[test]
fn test_wrong_token() {
    setup("8598", 0);

    let authentication_result = keycloak_validation::verify(
        "xyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyz",
        "http://localhost:8598",
        "my-realm",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_err());
}

#[test]
fn test_wrong_host() {
    setup("8597", 0);

    let authentication_result = keycloak_validation::verify(
        "012345678901234567890123456789",
        "http://8.8.8.8:8597",
        "my-realm",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_err());
}

#[test]
fn test_wrong_realm() {
    setup("8596", 0);

    let authentication_result = keycloak_validation::verify(
        "012345678901234567890123456789",
        "http://localhost:8596",
        "this-realm-does-not-exist",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_err());
}

#[test]
fn test_succes_plain() {
    setup("8595", 0);

    let authentication_result = keycloak_validation::verify(
        "012345678901234567890123456789",
        "http://localhost:8595",
        "my-realm",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_ok());
}

#[test]
fn test_succes_bearer() {
    setup("8594", 0);

    let authentication_result = keycloak_validation::verify(
        "Bearer 012345678901234567890123456789",
        "http://localhost:8594",
        "my-realm",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_ok());
}

#[test]
fn test_succes_authorization_bearer() {
    setup("8593", 0);

    let authentication_result = keycloak_validation::verify(
        "Authorization: Bearer 012345678901234567890123456789",
        "http://localhost:8593",
        "my-realm",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_ok());
}

#[test]
fn test_timeout() {
    setup("8592", 4000);

    let authentication_result = keycloak_validation::verify(
        "012345678901234567890123456789",
        "http://localhost:8592",
        "my-realm",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_err());
}
