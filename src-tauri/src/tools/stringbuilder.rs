#[tauri::command]
pub fn convert(text: &str) -> String {
    let mut sb = String::new();
    sb.push_str("var sb = new StringBuilder();\n");
    text.split('\n').for_each(|line| {
        sb.push_str(&format!(
            "sb.AppendLine(\"{}\");\n",
            line.replace('\r', "")
        ));
    });
    sb.remove(sb.len() - 1);
    sb
}

#[tauri::command]
pub fn revert(sb: &str) -> String {
    let mut text = String::new();
    sb.split('\n').for_each(|line| {
        if let Some(start) = line.find('"') {
            if let Some(end) = line.rfind('"') {
                if start != end {
                    let extracted = &line[start + 1..end];
                    text.push_str(extracted);
                    text.push('\n');
                }
            }
        }
    });
    if !text.is_empty() {
        text.remove(text.len() - 1);
    }
    text
}
