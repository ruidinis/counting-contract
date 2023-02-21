use serde::{Deserialize, Serialize};


// GEt a Message
#[derive(Deserialize, Serialize, Clone , Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Value {},
    Incremented { value: u64 },
}

// return response message
#[derive(Deserialize, Serialize, Clone , Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct  ValueResp {
    pub value: u64,
}