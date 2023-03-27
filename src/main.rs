mod auth;
mod auth_util;

fn main() {
    auth_util::save_swm_auth();

    let auth = auth::Authenticator::new("DeathBlows".to_string(), "".to_string());

    if let Ok((hwid, result, message)) = auth.authenticate() {
        println!("HWID: {}", hwid);
        println!("Result: {:?}", result);
        println!("Message: {}", message);
    } else {
        println!("Failed to authenticate");
    }
}
