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

#[derive(Serialize)]
#[serde(tag = "action", content = "data", rename_all = "lowercase")]
pub enum RequestPayload {
    Chat(ChatRequestData),
    Move(MoveRequestData)
}

#[derive(Deserialize)]
#[serde(tag = "action", content = "data", rename_all = "lowercase")]
pub enum ResponsePayload {
    Chat(ChatResponseData),
    Move(MoveResponseData)
}
