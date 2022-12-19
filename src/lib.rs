use serde::{Deserialize, Serialize};
mod behavior;
pub mod telegram_auth;

pub use behavior::Angle;
pub use uuid::Uuid;

pub type Time = u64;

pub use telegram_auth::{UserAuth, UserInfo};

pub const LEN_LIMIT_TITLE: usize = 512;
pub const LEN_LIMIT_DESCRIPTION: usize = 4096;
pub const LEN_LIMIT_CONTENT: usize = 4096;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(try_from = "i32", into = "i32")]
pub struct Longitude(i32);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(try_from = "i32", into = "i32")]
pub struct Latitude(i32);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    pub lat: Latitude,
    pub lng: Longitude,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Area {
    pub lat: Latitude,
    pub lng: Longitude,
    pub radius: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostPop {
    pub title: String,
    pub description: String,
    pub user: UserAuth,
    pub location: Location,
    pub expire: Time,
}

pub type ListPops = Vec<Uuid>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetPop {
    pub title: String,
    pub description: String,
    pub user: UserInfo,
    pub location: Location,
    pub expire: Time,
    pub created: Time,
    pub peps: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostPep {
    pub content: String,
    pub user: UserAuth,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetPep {
    pub content: String,
    pub user: UserInfo,
    pub created: Time,
}
