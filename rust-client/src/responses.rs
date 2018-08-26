/// The body returned from the identity service's `/validate` endpoint when a 400 error is the
/// status.
#[derive(Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "type")]
pub enum Validate400Response {
    /// The token had expired.
    #[serde(rename = "expired")]
    Expired,

    /// The token was structurally invalid.
    #[serde(rename = "invalid")]
    Invalid,
}

#[cfg(test)]
mod tests {
    use super::Validate400Response;
    use serde_json::from_str;

    macro_rules! test {
        (deser, $s:expr, $r:expr) => {
            assert_eq!(from_str::<Validate400Response>($s).unwrap(), $r);
        };
    }

    #[test]
    fn deserialize() {
        test!(deser, r#"{"type":"expired"}"#, Validate400Response::Expired);
        test!(deser, r#"{"type":"invalid"}"#, Validate400Response::Invalid);
    }
}
