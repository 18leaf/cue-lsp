//! # Module for Json RPC (version 2 for LSP spec)
//! Encoding/Decoding. Example Format From LSP spec below
//! ## Response
//! ```json
//!     Content-Length: ...\r\n
//!     \r\n
//!    {
//!            "jsonrpc": "2.0",
//!            "id": 1,
//!            "method": "textDocument/completion",
//!            "params": {
//!                    ...
//!            }
//!     }
//! ```
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};

//TODO
// Define Params Better

/// Struct for JsonRpc Request
/// this is what we will recieve
/// from the IDE
#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct JsonRpcResponse {
    pub testing: bool,
}

/// decodes an array of bytes (Json not Header)
/// and returns JsonRpcRequest type
/// TODO on caller -> get content header length + send
/// message as only the json part
pub fn decode_message(msg: &[u8]) -> Result<JsonRpcRequest> {
    serde_json::from_slice(msg)
}

/// Encode a Message (Request/Response)
/// To Json using serde.
pub fn encode_message(msg: JsonRpcResponse) -> Result<String> {
    // encode as a Json String
    // error panics here
    let encoded_content = match serde_json::to_string(&msg) {
        Ok(ec) => ec,
        Err(_) => panic!("String::from(\"<MAJOR ERROR HERE>\")"),
    };

    // find how many bytes string message has +
    // add to content length/header
    Ok(format! {"Content-Length: {}\r\n\r\n{encoded_content}", encoded_content.len()})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decode_message_works() -> Result<()> {
        let in_bytes = br#"{
    "jsonrpc": "2.0",
    "id" : 1,
    "method": "textDocument/definition",
    "params": {
        "textDocument": {
            "uri": "file:///p%3A/mseng/VSCode/Playgrounds/cpp/use.cpp"
        },
        "position": {
            "line": 3,
            "character": 12
        }
    }
}"#;

        let params_bytes = br#"{
        "textDocument": {
            "uri": "file:///p%3A/mseng/VSCode/Playgrounds/cpp/use.cpp"
        },
        "position": {
            "line": 3,
            "character": 12
        }
    }
"#;
        let result = decode_message(in_bytes)?;

        let expected = JsonRpcRequest {
            jsonrpc: String::from("2.0"),
            id: Some(Value::from(1)),
            method: String::from("textDocument/definition"),
            params: Some(serde_json::from_slice(params_bytes)?),
        };

        dbg!(&expected);

        dbg!(&result);

        assert_eq!(result, expected);
        Ok(())
    }

    #[test]
    fn encode_message_works() -> Result<()> {
        let expected = "Content-Length: 16\r\n\r\n{\"testing\":true}";

        let msg = JsonRpcResponse { testing: true };
        let encoded_as_json = encode_message(msg)?;

        assert_eq!(encoded_as_json, expected);
        Ok(())
    }
}
