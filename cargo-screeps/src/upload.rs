use std::{collections::HashMap, fs, io::Read, path::Path};

use {base64, failure, reqwest, serde_json};

use config::{Authentication, Configuration};

pub fn upload(root: &Path, config: &Configuration) -> Result<(), failure::Error> {
    let upload_config = config.upload.as_ref().ok_or_else(|| {
        format_err!("must include [upload] section in configuration to deploy using upload")
    })?;

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
        if upload_config.ssl { "https" } else { "http" },
        upload_config.hostname,
        upload_config.port,
        if upload_config.ptr {
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

    let mut response = authenticate(client.post(&url), &upload_config.authentication)
        .json(&RequestData {
            modules: files,
            branch: upload_config.branch.clone(),
        })
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
            upload_config.branch,
            response.url(),
            s
        );
    }

    Ok(())
}

fn authenticate(
    request: reqwest::RequestBuilder,
    authentication: &Authentication,
) -> reqwest::RequestBuilder {
    match authentication {
        Authentication::Token(ref token) => request.header("X-Token", token.as_str()),
        Authentication::Basic {
            ref username,
            ref password,
        } => request.basic_auth(username, Some(password)),
    }
}
