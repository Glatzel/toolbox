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

#[cfg(test)]
mod tests {
    use super::*;

    // ===== Resize =====

    #[test]
    fn parse_resize() {
        let msg = r#"{"kind":"resize","cols":120,"rows":40}"#;
        let parsed = ReceiveMsg::parse(msg).unwrap();
        assert!(matches!(
            parsed,
            ReceiveMsg::Resize(ResizeMsg {
                cols: 120,
                rows: 40
            })
        ));
    }

    #[test]
    fn parse_resize_zero_dimensions() {
        let msg = r#"{"kind":"resize","cols":0,"rows":0}"#;
        let parsed = ReceiveMsg::parse(msg).unwrap();
        assert!(matches!(
            parsed,
            ReceiveMsg::Resize(ResizeMsg { cols: 0, rows: 0 })
        ));
    }

    #[test]
    fn parse_resize_missing_cols() {
        let msg = r#"{"kind":"resize","rows":40}"#;
        assert!(ReceiveMsg::parse(msg).is_err());
    }

    #[test]
    fn parse_resize_missing_rows() {
        let msg = r#"{"kind":"resize","cols":120}"#;
        assert!(ReceiveMsg::parse(msg).is_err());
    }

    // ===== Input =====

    #[test]
    fn parse_plain_text_as_input() {
        let msg = "hello";
        let parsed = ReceiveMsg::parse(msg).unwrap();
        let ReceiveMsg::Input(input) = parsed else {
            panic!("expected Input")
        };
        assert_eq!(input.data, "hello");
    }

    #[test]
    fn parse_input_preserves_data() {
        let msg = "ls -la\n";
        let parsed = ReceiveMsg::parse(msg).unwrap();
        let ReceiveMsg::Input(input) = parsed else {
            panic!("expected Input")
        };
        assert_eq!(input.data, "ls -la\n");
    }

    #[test]
    fn parse_input_special_chars() {
        let msg = "\x03"; // Ctrl+C
        let parsed = ReceiveMsg::parse(msg).unwrap();
        let ReceiveMsg::Input(input) = parsed else {
            panic!("expected Input")
        };
        assert_eq!(input.data, "\x03");
    }

    #[test]
    fn parse_input_empty_string() {
        let msg = "";
        let parsed = ReceiveMsg::parse(msg).unwrap();
        let ReceiveMsg::Input(input) = parsed else {
            panic!("expected Input")
        };
        assert_eq!(input.data, "");
    }

    // ===== Unknown JSON kind =====

    #[test]
    fn parse_unknown_kind_errors() {
        let msg = r#"{"kind":"unknown"}"#;
        assert!(ReceiveMsg::parse(msg).is_err());
    }

    #[test]
    fn parse_json_missing_kind_errors() {
        let msg = r#"{"cols":80,"rows":24}"#;
        assert!(ReceiveMsg::parse(msg).is_err());
    }

    #[test]
    fn parse_json_null_kind_errors() {
        let msg = r#"{"kind":null,"cols":80,"rows":24}"#;
        assert!(ReceiveMsg::parse(msg).is_err());
    }
}
