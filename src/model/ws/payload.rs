use serde::{Serialize, Deserialize};

use super::super::{
    chat::payload::{
        RequestPayload as ChatRequestData,
        ResponsePayload as ChatResponseData
    },
    movement::payload::{
        RequestPayload as MoveRequestData,
        ResponsePayload as MoveResponseData
    }
};

#[derive(Deserialize)]
pub struct LogoutResponseData {
    id: usize
}

impl LogoutResponseData {
    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Deserialize)]
pub struct LeaveTownResponseData {
    id: usize
}

impl LeaveTownResponseData {
    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Deserialize)]
pub struct LeaveRegionResponseData {
    id: usize
}

impl LeaveRegionResponseData {
    pub fn get_id(&self) -> usize {
        self.id
    }
}

#[derive(Serialize)]
#[serde(tag = "action", content = "data", rename_all = "lowercase")]
pub enum RequestPayload {
    Chat(ChatRequestData),
    Move(MoveRequestData)
}

#[derive(Deserialize)]
#[serde(tag = "action", content = "data", rename_all = "snake_case")]
pub enum ResponsePayload {
    Chat(ChatResponseData),
    Move(MoveResponseData),
    Logout(LogoutResponseData),
    LeaveTown(LeaveTownResponseData),
    LeaveRegion(LeaveRegionResponseData),
    Ping
}
