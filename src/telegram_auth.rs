use std::{path::Path, time::SystemTime};

use serde::{Deserialize, Serialize};

pub type Token = [u8; 32];

pub const FAKE_TOKEN: Token = [0; 32];

#[derive(Debug, Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: u64,
    pub first_name: String,
    pub last_name: String,
    pub photo_url: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAuth {
    pub id: u64,
    pub auth_date: u64,
    pub first_name: String,
    pub last_name: String,
    pub photo_url: String,
    pub hash: String,
}

impl UserAuth {
    /// Check authentication
    ///
    /// If `fake_auth` feature is activated, returns `true` when `.hash == "FAKE_AUTH"`
    pub fn is_valid(&self, token: Token) -> bool {
        if cfg!(feature = "fake_auth") {
            return self.hash == "FAKE_AUTH";
        };
        use hex::FromHex;
        if let Ok(hash) = <[u8; 32]>::from_hex(&self.hash) {
            let fields = [
                format!("auth_date={}", self.auth_date),
                format!("first_name={}", self.first_name),
                format!("id={}", self.id),
                format!("last_name={}", self.last_name),
                format!("photo_url={}", self.photo_url),
            ]
            .join("\n");
            hash == hmac_sha256::HMAC::mac(fields, token)
        } else {
            false
        }
    }
}

pub fn token_from_file(path: impl AsRef<Path>) -> Token {
    let hexa = std::fs::read_to_string(path).expect("could not open telegram token file");
    hex::FromHex::from_hex(&hexa).expect("telegram token should be 32 bytes in hexadecimal")
}

impl TryFrom<(Token, UserAuth)> for UserInfo {
    type Error = ();

    fn try_from((token, user): (Token, UserAuth)) -> Result<Self, Self::Error> {
        if user.is_valid(token) {
            Ok(Self {
                id: user.id,
                first_name: user.first_name,
                last_name: user.last_name,
                photo_url: user.photo_url,
            })
        } else {
            Err(())
        }
    }
}

impl UserInfo {
    pub fn fake_auth(self) -> UserAuth {
        let UserInfo {
            id,
            first_name,
            last_name,
            photo_url,
        } = self;
        let auth_date = SystemTime::UNIX_EPOCH.elapsed().unwrap().as_secs();
        let hash = "FAKE_AUTH".into();
        UserAuth {
            id,
            auth_date,
            first_name,
            last_name,
            photo_url,
            hash,
        }
    }
}
