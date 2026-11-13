// use serde::{Deserialize, Serialize};
// use serde_json::Value;
//
// #[derive(Deserialize, Debug)]
// pub struct Model {
//     pub id: String,
// }
//
// #[derive(Deserialize, Debug)]
// pub struct ModelList {
//     pub data: Vec<Model>,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct FnCall {
//     pub name: String,
//     pub arguments: String, // JSON *string*, not object
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct ToolCall {
//     pub id: String,
//     #[serde(rename = "type")]
//     pub kind: String,
//     pub function: FnCall,
// }
//
// #[derive(Deserialize, Serialize, Clone, Debug)]
// pub struct Msg {
//     pub role: String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub content: Option<String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub tool_calls: Option<Vec<ToolCall>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub tool_call_id: Option<String>,
// }
//
// impl Msg {
//     pub fn user(s: impl Into<String>) -> Self {
//         Msg {
//             role: "user".into(),
//             content: Some(s.into()),
//             tool_calls: None,
//             tool_call_id: None,
//         }
//     }
//
//     #[allow(dead_code)] // used once the loop exists
//     pub fn tool_result(id: impl Into<String>, s: impl Into<String>) -> Self {
//         Msg {
//             role: "tool".into(),
//             content: Some(s.into()),
//             tool_calls: None,
//             tool_call_id: Some(id.into()),
//         }
//     }
// }
//
// #[derive(Serialize)]
// pub struct Thinking<'a> {
//     #[serde(rename = "type")]
//     kind: &'a str, // "enabled" | "disabled"
// }
//
// impl Thinking<'_> {
//     pub fn set(enabled: bool) -> Thinking<'static> {
//         Thinking {
//             kind: if enabled { "enabled" } else { "disabled" },
//         }
//     }
// }
//
// #[derive(Serialize)]
// pub struct ChatReq<'a> {
//     pub model: &'a str,
//     pub messages: Vec<Msg>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub thinking: Option<Thinking<'a>>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub temperature: Option<f32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub reasoning_effort: Option<&'a str>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub top_p: Option<f32>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub tools: Option<&'a Value>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub tool_choice: Option<&'a Value>,
// }
//
// #[derive(Deserialize, Debug)]
// pub struct OutMsg {
//     pub content: Option<String>, // null when the model only calls tools
//     pub tool_calls: Option<Vec<ToolCall>>, // must be declared, or serde drops it silently
// }
//
// #[derive(Deserialize, Debug)]
// pub struct Choice {
//     pub message: OutMsg,
//     pub finish_reason: Option<String>,
// }
//
// #[derive(Deserialize, Debug)]
// pub struct ChatResp {
//     pub choices: Vec<Choice>,
// }
