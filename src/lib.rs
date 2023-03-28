mod auth;
mod auth_util;

pub use auth::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth() {
        let auth = auth::Authenticator::new(
            "DeathBlows".to_string(),
            "NO PASSWORD FOR U RETARD LOL! THIS IS AN ENV VARIABLE XD".to_string(),
        );

        if let Ok((hwid, result, message)) = auth.authenticate() {
            println!("HWID: {}", hwid);
            println!("Result: {:?}", result);
            println!("Message: {}", message);
        } else {
            println!("Failed to authenticate");
        }
    }
}
