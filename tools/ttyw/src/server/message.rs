use std::str::FromStr;

use serde::Deserialize;
pub enum ReceiveMsg {
    Resize(ResizeMsg),
    Input(InputMsg),
}
impl ReceiveMsg {
    pub fn parse(msg: &str) -> mischief::Result<Self> {
        match serde_json::Value::from_str(msg) {
            Ok(msg) => match msg["kind"].as_str() {
                Some("resize") => Ok(Self::Resize(serde_json::from_value::<ResizeMsg>(msg)?)),
                _ => Err(mischief::mischief!("Unknown message: {}", msg)),
            },
            Err(_) => Ok(Self::Input(InputMsg {
                data: msg.to_string(),
            })),
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
