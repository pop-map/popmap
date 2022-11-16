use serde::{Deserialize, Serialize};
mod behavior;

pub use uuid::Uuid;

pub type Time = u64;

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Degree(i16, u8, u8);

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Location {
    pub lat: Degree,
    pub lng: Degree,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Area {
    pub lat: Degree,
    pub lng: Degree,
    pub radius: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostPop {
    pub title: String,
    pub description: String,
    pub location: Location,
    pub expire: Time,
}

pub type ListPops = Vec<Uuid>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetPop {
    pub title: String,
    pub description: String,
    pub location: Location,
    pub expire: Time,
    pub created: Time,
    pub peps: usize,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostPep {
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GetPep {
    pub content: String,
    pub created: Time,
}
