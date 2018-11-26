# Keycloak validation

This crate allows the validation of a keycloak access token.

## Example

```rust
extern crate keycloak_validation;

fn main() {

    let validation = keycloak_validation::verify(
        "0123456789", // or "Bearer 0123456789" or "Authorization: Bearer 0123456789"
        "https://www.my-keycloak-website.org",
        "realm-name",
    );

    match validation {
        Ok(user_info) => println!("User info: {}", user_info),
        Err(error) => println!("A validation error occurred: {}", error),
    };
}
```

More [examples](examples/)