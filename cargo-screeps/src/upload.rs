use std::collections::HashMap;
use std::path::Path;
use std::io::Read;
use std::fs;

use {failure, reqwest, serde_json, base64};

use setup::Configuration;

pub fn upload(root: &Path, config: Configuration) -> Result<(), failure::Error> {
    let target_dir = root.join("target");

    let mut files = HashMap::new();
    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        if let (Some(name), Some(extension)) = (path.file_stem(), path.extension()) {
            let contents = if extension == "js" {
                let data = {
                    let mut buf = String::new();
                    fs::File::open(&path)?.read_to_string(&mut buf)?;
                    buf
                };
                serde_json::Value::String(data)
            } else if extension == "wasm" {
                let data = {
                    let mut buf = Vec::new();
                    fs::File::open(&path)?.read_to_end(&mut buf)?;
                    buf
                };
                let data = base64::encode(&data);
                json!({ "binary": data })
            } else {
                continue;
            };

            files.insert(name.to_string_lossy().into_owned(), contents);
        }
    }

    let client = reqwest::Client::new();

    let url = format!(
        "{}://{}:{}/{}",
        if config.ssl { "https" } else { "http" },
        config.hostname,
        config.port,
        if config.ptr {
            "ptr/api/user/code"
        } else {
            "api/user/code"
        }
    );

    #[derive(Serialize)]
    struct RequestData {
        modules: HashMap<String, serde_json::Value>,
        branch: String,
    }

    let mut response = client
        .post(&*url)
        .basic_auth(config.username, Some(config.password))
        .header(reqwest::header::ContentType::json())
        .body(serde_json::to_string(&RequestData {
            modules: files,
            branch: config.branch.clone(),
        })?)
        .send()?;

    let response_text = response.text()?;

    ensure!(
        response.status().is_success(),
        "uploading to '{}' failed: {}",
        response.url(),
        response_text,
    );

    debug!("upload finished: {}", response_text);
    debug!("response: {:#?}", response);

    let response_json: serde_json::Value = serde_json::from_str(&response_text)?;

    if let Some(s) = response_json.get("error") {
        bail!(
            "error sending to branch '{}' of '{}': {}",
            config.branch,
            response.url(),
            s
        );
    }

    Ok(())
}
