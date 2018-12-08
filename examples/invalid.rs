use keycloak_validation;

fn main() {
    let authentication_result = keycloak_validation::verify(
        "012345678901234567890123456789",
        "https://login-dev.scoutsengidsenvlaanderen.be",
        "my-realm",
        std::time::Duration::from_secs(3),
    );

    assert!(authentication_result.is_err());
    println!("{}", authentication_result.unwrap_err());
}
