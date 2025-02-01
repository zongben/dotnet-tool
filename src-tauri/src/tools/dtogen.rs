use std::{env, path::PathBuf, process::Command};

#[tauri::command]
pub fn connect() -> Result<String, String> {
    let exe_dir = env::current_exe()
        .map_err(|e| e.to_string())?
        .parent()
        .map(PathBuf::from)
        .ok_or_else(|| "no parent".to_string())?;

    let dtgo_path = PathBuf::from(exe_dir).join("dtgo");

    let output = Command::new(dtgo_path)
        .output()
        .map_err(|e| e.to_string())?;

    Ok(String::from_utf8(output.stdout).map_err(|e| e.to_string())?)
}
