use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub src: String,
    pub dest: String,
    pub body: Option<Payload>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Init {
    pub msg_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitOk {
    pub in_reply_to: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Echo {
    pub msg_id: usize,
    pub echo: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EchoOk {
    pub msg_id: usize,
    pub echo: String,
    pub in_reply_to: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Generate {
    pub msg_id: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GenerateOk {
    pub in_reply_to: usize,
    pub id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Payload {
    Echo(Echo),
    EchoOk(EchoOk),
    Init(Init),
    InitOk(InitOk),
    Generate(Generate),
    GenerateOk(GenerateOk),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payload_init() {
        let payload_str = r#"{
            "type": "init",
            "msg_id": 1
            }
        "#;
        let payload = serde_json::from_str::<Payload>(payload_str).expect("error");
        assert_eq!(payload, Payload::Init(Init { msg_id: 1 }));
    }

    #[test]
    fn test_payload_echo() {
        let payload_str = r#"{
            "type": "echo",
            "echo": "hello",
            "msg_id": 1
            }
        "#;
        let payload = serde_json::from_str::<Payload>(payload_str).expect("error");
        assert_eq!(
            payload,
            Payload::Echo(Echo {
                msg_id: 1,
                echo: "hello".to_string()
            })
        );
    }
}
