use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueResult<T> {
    pub value: T,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ValueData<T> {
    pub data: T,
}
