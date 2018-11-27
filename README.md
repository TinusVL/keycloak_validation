# Keycloak validation

This crate allows the validation of a keycloak access token. Local validation of a token is hard to get right, so the validation is delegated to the keycloak server.

## Example

```rust
extern crate keycloak_validation;

fn main() {

    let authentication_result = keycloak_validation::verify(
        "0123456789", // or "Bearer 0123456789" or "Authorization: Bearer 0123456789"
        "https://www.my-keycloak-website.org",
        "my-realm-name",
        std::time::Duration::from_secs(3),
    );

    match authentication_result {
        Ok(authentication) => println!("User info: {}", authentication.user_info),
        Err(error) => println!("A validation error occurred: {}", error),
    };
}
```

More [examples](examples/)
