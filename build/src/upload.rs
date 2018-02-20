use std::collections::HashMap;
use std::fs;

use {failure, find_folder, reqwest, serde_json, base64};

use setup::Configuration;

pub fn upload(config: Configuration) -> Result<(), failure::Error> {
    let target_dir = find_folder::Search::Parents(2)
        .for_folder("source")?
        .join("../target");
    let mut files = HashMap::new();
    for entry in fs::read_dir(target_dir)? {
        let entry = entry?;
        let path = entry.path();

        ensure!(
            entry.file_type()?.is_file(),
            "non-file found in 'target' dir: {}",
            path.display()
        );

        if let (Some(name), Some(extension)) = (path.file_stem(), path.extension()) {
            let contents = if extension == "js" {
                let data = fs::read_string(&path)?;
                serde_json::Value::String(data)
            } else if extension == "wasm" {
                let data = base64::encode(&fs::read(&path)?);
                json!({ "binary": data })
            } else {
                bail!("non-js non-wasm file found in target/");
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
