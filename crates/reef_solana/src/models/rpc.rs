use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueResult<T> {
    pub value: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueData<T> {
    pub data: T,
    pub owner: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parsed<T> {
    pub parsed: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info<T> {
    pub info: T,
}
