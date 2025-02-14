use std::{env, path::PathBuf, process::Command};

#[derive(Debug, serde::Deserialize)]
pub struct ConnectArgs {
    host: String,
    init_db: String,
    username: String,
    password: String,
}

#[tauri::command]
pub fn connect(args: ConnectArgs) -> Result<String, String> {
    let exe_dir = env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| "no parent".to_string())?;

    let dtgo_path = PathBuf::from(exe_dir).join("dtgo");

    let output = Command::new(dtgo_path)
        .arg("mssql")
        .arg(format!("--host={}", args.host))
        .arg(format!("--init-db={}", args.init_db))
        .arg(format!("--username={}", args.username))
        .arg(format!("--password={}", args.password))
        .output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8(output.stdout).map_err(|e| e.to_string())?)
}
