use serde::{Deserialize, Serialize};
mod behavior;

pub use behavior::Angle;
pub use uuid::Uuid;

pub type Time = u64;

pub use telegram_auth::{UserAuth, UserInfo};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(try_from = "(i16, u8, u8)", into = "(i16, u8, u8)")]
pub struct Longitude {
    deg: i16,
    min: u8,
    sec: u8,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(try_from = "(i16, u8, u8)", into = "(i16, u8, u8)")]
pub struct Latitude {
    deg: i16,
    min: u8,
    sec: u8,
}

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
