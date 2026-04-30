use std::str::FromStr;

use mischief::IntoMischief;
use serde::Deserialize;
pub enum ReceiveMsg {
    Resize(ResizeMsg),
    Command(InputMsg),
}
impl ReceiveMsg {
    pub fn parse(msg: &str) -> mischief::Result<Self> {
        let msg = serde_json::Value::from_str(msg).into_mischief()?;
        match msg["kind"].as_str() {
            Some("resize") => Ok(Self::Resize(serde_json::from_value::<ResizeMsg>(msg)?)),
            Some("input") => Ok(Self::Command(serde_json::from_value::<InputMsg>(msg)?)),
            _ => Err(mischief::mischief!("Unknown message: {}", msg)),
        }
    }
}
#[derive(Deserialize)]
pub struct ResizeMsg {
    pub cols: u16,
    pub rows: u16,
}
#[derive(Deserialize)]
pub struct InputMsg {
    pub data: String,
}
