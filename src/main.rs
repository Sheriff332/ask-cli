use serde_json::{Value, json};
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::time::Duration;

use clap::Parser;
use dirs::{config_dir, data_dir};
use env::{CONF, CUR_KEY};

mod api_defs;
mod env;

/// Posts a chat request and returns the raw body, printing the server's
/// error message instead of hiding it behind a bare status code.
async fn post_chat(
    client: &reqwest::Client,
    msgs_path: PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let messages: Vec<Value> = fs::read_to_string(&msgs_path)
        .unwrap_or_default()
        .lines()
        .filter(|l| !l.trim().is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    let res = client
        .post(format!(
            "{}/chat/completions",
            CONF.get("provider-url")
                .and_then(|v| v.as_str())
                .ok_or("config missing 'provider-url'")?
        ))
        .bearer_auth(CUR_KEY.as_str())
        .json(&json!({
        "model": CONF.get("current-model").ok_or("current model not set")?,
        "messages": messages,
        "temperature": 1.0
    }))
        .send()
        .await?;

    let status = res.status();
    let body = res.text().await?;
    if !status.is_success() {
        return Err(format!("{status}: {body}").into());
    }
    Ok(body)
}

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    msg: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(config_dir().ok_or("config dir missing")?.join("ask"))?;
    fs::create_dir_all(data_dir().ok_or("data dir missing")?.join("ask"))?;

    let args = Cli::parse();

    let msg = args.msg.ok_or("ask what, fatass?")?;
    let mut msgs_writer = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(
            data_dir()
                .ok_or("data dir missing")?
                .join("ask")
                .join("msgs.jsonl"),
        )?;
    writeln!(msgs_writer, "{}",
             json!({
                "role": "user",
                "content": msg,
             }))?;

    // let data = fs::read_to_string(&path).unwrap_or_default();
    // let messages: Vec<Value> = data
    //     .lines()
    //     .filter(|line| !line.trim().is_empty())
    //     .filter_map(|line| serde_json::from_str(line).ok())
    //     .collect();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(90))
        .build()?;

    // let model = CONF
    //     .get("current-model")
    //     .and_then(|v| v.as_str())
    //     .ok_or("config missing 'current-model'")?; DONT REMOVE THIS

    // let models: ModelList = client
    //     .get(format!(
    //         "{}/models",
    //         CONF.get("provider-url")
    //             .and_then(|v| v.as_str())
    //             .ok_or("config missing 'provider-url'")?
    //     ))
    //     .bearer_auth(CUR_KEY.as_str())
    //     .send()
    //     .await?
    //     .error_for_status()? // fine here: /models has no request body to misdeclare
    //     .json()
    //     .await?;
    //
    // if !models.data.iter().any(|m| m.id == model) {
    //     eprintln!("model {model:?} not available. got:");
    //     for m in &models.data {
    //         eprintln!("  {}", m.id);
    //     }
    //     return Ok(());
    // }

    //chat call
    // let req = ChatReq {
    //     model,
    //     messages: vec![Msg::user(msg.as_str())],
    //     thinking: Some(Thinking::set(false)), // off: sampling knobs actually work
    //     temperature: Some(0.3),
    //     reasoning_effort: None,
    //     top_p: None,
    //     tools: Some(&TOOLS),
    //     tool_choice: None,
    // };

    let body = post_chat(&client, data_dir()
        .ok_or("data dir missing")?
        .join("ask")
        .join("msgs.jsonl")).await?;
    let content = serde_json::from_str::<Value>(&body)?["choices"][0]["message"]["content"]
        .as_str()
        .ok_or("missing content")?
        .to_string();
    writeln!(msgs_writer, "{}", json!({
        "role": "assistant",
        "content": content
    }))?;
    println!("{content}");
    Ok(())
    // let resp: ChatResp = serde_json::from_str(&body)?;
    //
    // let choice = resp.choices.first().ok_or("response had no choices")?;
    //
    // if let Some(calls) = &choice.message.tool_calls {
    //     for call in calls {
    //         let args = serde_json::from_str::<Value>(&call.function.arguments);
    //         if let Ok(args) = args {
    //             if let Some(path) = args.get("path") {
    //                 dbg!(path.to_string());
    //             }
    //         }
    //     }
    //     eprintln!("model asked for {} tool call(s)", calls.len());
    // }
    // println!(
    //     "{}",
    //     choice.message.content.as_deref().unwrap_or("<no content>")
    // );
    // let _ = &choice.finish_reason;
    //
    // Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     fs::create_dir_all(format!(
//         "{}/ask/",
//         config_dir()
//             .expect("config dir not found")
//             .to_string_lossy()
//     ))?;
//
//     let args = Cli::parse();
//     let msg = args.msg.expect("ask what, fatass?");
//     let client = reqwest::Client::builder()
//         .timeout(Duration::from_secs(120))
//         .build()?;
//
//     let model = CONF.get("current-model").and_then(|v| v.as_str()).ok_or("config missing 'current-model'")?;
//
//     let models: ModelList = client
//         .get(format!("{}/models", CONF.get("provider-url").and_then(|v| v.as_str()).ok_or("config missing 'provider-url'")?))
//         .bearer_auth(CUR_KEY.as_str())
//         .send()
//         .await?
//         .error_for_status()? // fine here: /models has no request body to misdeclare
//         .json()
//         .await?;
//
//     if !models.data.iter().any(|m| m.id == model) {
//         eprintln!("model {model:?} not available. got:");
//         for m in &models.data {
//             eprintln!("  {}", m.id);
//         }
//         return Ok(());
//     }
//
//     //chat call
//     let req = ChatReq {
//         model,
//         messages: vec![Msg::user(msg.as_str())],
//         thinking: Some(Thinking::set(false)), // off: sampling knobs actually work
//         temperature: Some(0.3),
//         reasoning_effort: None,
//         top_p: None,
//         tools: Some(&TOOLS),
//         tool_choice: None,
//     };
//
//     let body = post_chat(&client, &req).await?;
//     let resp: ChatResp = serde_json::from_str(&body)?;
//
//     let choice = resp.choices.first().ok_or("response had no choices")?;
//
//     if let Some(calls) = &choice.message.tool_calls {
//         for call in calls {
//             let args = serde_json::from_str::<Value>(&call.function.arguments);
//             if let Ok(args) = args {
//                 if let Some(path) = args.get("path") {
//                     dbg!(path.to_string());
//                 }
//             }
//         }
//         eprintln!("model asked for {} tool call(s)", calls.len());
//     }
//     println!(
//         "{}",
//         choice.message.content.as_deref().unwrap_or("<no content>")
//     );
//     let _ = &choice.finish_reason;
//
//     Ok(())
// }
