mod auth;

pub use self::{auth::AuthCodes, auth::Authenticator};

#[cfg(test)]
mod tests {
    #[test]
    fn test_auth() {
        assert_eq!(1, 1)
    }
}
