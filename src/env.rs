use dirs::config_dir;
use std::fs;
use std::io::Write;
use std::sync::LazyLock;
use toml::Table;

// pub static TOOLS: LazyLock<Value> = LazyLock::new(|| {
//     json!([{
//         "type": "function",
//         "function": {
//             "name": "read_file",
//             "description": "Read a UTF-8 text file from disk.",
//             "parameters": {
//                 "type": "object",
//                 "properties": { "path": { "type": "string", "description": "Relative path" } },
//                 "required": ["path"],
//                 "additionalProperties": false
//             }
//         }
//     }])
// });
pub static KEYS: LazyLock<Table> = LazyLock::new(|| {
    let config_path = format!(
        "{}/ask/keys.toml",
        config_dir()
            .expect("config dir not found")
            .to_string_lossy()
    );
    match fs::File::create_new(&config_path) {
        Ok(mut f) => {
            writeln!(f, r#"key = """#).expect("write to file");
        }
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => {
            eprintln!("could not create {}: {e}", config_path);
        }
        _ => {}
    }
    fs::read_to_string(&config_path)
        .expect("config read failed")
        .parse::<Table>()
        .expect("config parse failed")
});
pub static CUR_KEY: LazyLock<String> = LazyLock::new(|| {
    if let toml::Value::String(id) = CONF.get("current-key").expect("no key chosen in config") {
        KEYS.get(id)
            .and_then(|v| v.as_str())
            .ok_or(id)
            .expect("no key found for chosen id")
            .to_string()
    } else {
        panic!("chosen key is not a string");
    }
});
pub static CONF: LazyLock<Table> = LazyLock::new(|| {
    let config_path = format!(
        "{}/ask/config.toml",
        config_dir()
            .expect("config dir not found")
            .to_string_lossy()
    );
    match fs::File::create_new(&config_path) {
        Ok(mut f) => {
            writeln!(f, r#"provider-url = """#).expect("write to file failed");
            writeln!(f, r#"current-model = """#).expect("write to file failed");
            writeln!(f, r#"current-key = "key""#).expect("write to file failed");
        }
        Err(e) if e.kind() != std::io::ErrorKind::AlreadyExists => {
            eprintln!("could not create {}: {e}", config_path);
        }
        _ => {}
    }
    fs::read_to_string(&config_path)
        .expect("config read failed")
        .parse::<Table>()
        .expect("config parse failed")
});
