use std::{fs, os::unix::prelude::PermissionsExt, process::Command, str::FromStr};

use base64::{engine::general_purpose, Engine as _};

const SWMAUTH2_PATH: &'static str = "/Users/Shared/ScriptWare/SWMAuth2";

/// Codes that auth returns
#[derive(Debug, PartialEq)]
pub enum AuthCodes {
    NoSavedLogin,
    Emergency,
    BadCredentials,
    HwidChanged,
    BadSystemTime,
    TooManySystemChanges,
    AccountBlacklisted,
    BadServerResponse,
    InternalServerError,
    Success,
    UnknownError,
}

impl FromStr for AuthCodes {
    type Err = ();

    fn from_str(input: &str) -> Result<AuthCodes, Self::Err> {
        match input {
            "999" => Ok(AuthCodes::NoSavedLogin),
            "9999" => Ok(AuthCodes::Emergency),
            "1001" => Ok(AuthCodes::BadCredentials),
            "1002" => Ok(AuthCodes::HwidChanged),
            "1004" => Ok(AuthCodes::BadSystemTime),
            "1005" => Ok(AuthCodes::TooManySystemChanges),
            "1006" => Ok(AuthCodes::AccountBlacklisted),
            "998" | "false.998" => Ok(AuthCodes::BadServerResponse),
            "502" => Ok(AuthCodes::InternalServerError),
            _ => Ok(AuthCodes::Success),
        }
    }
}

pub struct Authenticator {
    username: String,
    password: String,
}

impl Authenticator {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn authenticate(&self) -> Result<(bool, AuthCodes, String), AuthCodes> {
        let output = self.spawn_authenticator();

        return self.parse_output(&output);
    }

    /// Spawns the SWMAuth2 process and returns the output.
    /// The output is the output of the SWMAuth2 process.
    fn spawn_authenticator(&self) -> String {
        let enc_username = general_purpose::STANDARD.encode(&self.username.as_bytes());
        let enc_password = general_purpose::STANDARD.encode(&self.password.as_bytes());

        // chmod the file to give perms
        fs::set_permissions(SWMAUTH2_PATH, fs::Permissions::from_mode(0o777))
            .expect("Failed to set permissions for SWMAuth2");

        let cmd = Command::new(SWMAUTH2_PATH)
            .args([enc_username, enc_password])
            .output()
            .expect("failed to execute process");

        let output = String::from_utf8_lossy(&cmd.stdout).to_string();

        output
    }

    fn parse_output(&self, output: &String) -> Result<(bool, AuthCodes, String), AuthCodes> {
        let mut unparsed_result = String::new();

        if let Some(index) = output.find("<result:") {
            if let Some(captures) = regex::Regex::new(r"<result:(.*?)>")
                .unwrap()
                .captures(&&output[index..])
            {
                unparsed_result = captures.get(1).unwrap().as_str().to_owned();
            }
        }

        // collect the result into a vector
        let split_result: Vec<&str> = unparsed_result.split('.').collect();

        // get the values from the vector
        let success = split_result.get(0).expect("Couldn't get success value");
        let non_enum_code = split_result.get(1);

        if non_enum_code.is_none() {
            return Err(AuthCodes::BadCredentials);
        }

        let token = split_result.get(2).unwrap();
        let result_code = AuthCodes::from_str(non_enum_code.expect("Couldln't convert to enum"));

        if let Err(_) = result_code {
            return Err(result_code.expect("Couldln't convert to enum"));
        } else if success == &"false" {
            return Err(result_code.expect("Success code error in result"));
        }

        let success_as_bool = success
            .parse::<bool>()
            .expect("Failed to parse success code as bool");

        return Ok((success_as_bool, result_code.unwrap(), token.to_string()));
    }
}
