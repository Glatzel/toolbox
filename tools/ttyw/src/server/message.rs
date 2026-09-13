

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "kind")]
pub enum ReceiveMsg {
    #[serde(rename = "resize")]
    Resize { cols: u16, rows: u16 },
    #[serde(rename = "input")]
    Input { data: String },
}

impl ReceiveMsg {
    pub fn parse(msg: &str) -> serde_json::Result<Self> { serde_json::from_str(msg) }
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
            ReceiveMsg::Resize {
                cols: 120,
                rows: 40
            }
        ));
    }

    #[test]
    fn parse_resize_zero_dimensions() {
        let msg = r#"{"kind":"resize","cols":0,"rows":0}"#;
        let parsed = ReceiveMsg::parse(msg).unwrap();
        assert!(matches!(parsed, ReceiveMsg::Resize { cols: 0, rows: 0 }));
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
        let ReceiveMsg::Input { data } = parsed else {
            panic!("expected Input")
        };
        assert_eq!(data, "hello");
    }

    #[test]
    fn parse_input_preserves_data() {
        let msg = "ls -la\n";
        let parsed = ReceiveMsg::parse(msg).unwrap();
        let ReceiveMsg::Input { data } = parsed else {
            panic!("expected Input")
        };
        assert_eq!(data, "ls -la\n");
    }

    #[test]
    fn parse_input_special_chars() {
        let msg = "\x03"; // Ctrl+C
        let parsed = ReceiveMsg::parse(msg).unwrap();
        let ReceiveMsg::Input { data } = parsed else {
            panic!("expected Input")
        };
        assert_eq!(data, "\x03");
    }

    #[test]
    fn parse_input_empty_string() {
        let msg = "";
        let parsed = ReceiveMsg::parse(msg).unwrap();
        let ReceiveMsg::Input { data } = parsed else {
            panic!("expected Input")
        };
        assert_eq!(data, "");
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
