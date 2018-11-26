extern crate keycloak_validation;

fn main() {
    let authentication_result = keycloak_validation::verify(
        "012345678901234567890123456789",
        "https://login-dev.scoutsengidsenvlaanderen.be",
        "scouts",
    );

    let error = authentication_result.unwrap_err();
    println!("{}", error);
    assert_eq!("Invalid token", error);
}
