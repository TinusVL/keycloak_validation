extern crate keycloak_validation;

fn main() {

    let user_info = keycloak_validation::verify(
        "012345678901234567890123456789",
        "https://login-dev.scoutsengidsenvlaanderen.be",
        "scouts",
    );

    println!("{}", user_info.unwrap_err());
}
