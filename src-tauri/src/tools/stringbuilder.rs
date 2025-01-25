#[tauri::command]
pub fn convert_text(text: &str) -> String {
    let mut sb = String::new();
    sb.push_str("StringBuilder sb = new StringBuilder();\n");
    text.split('\n').for_each(|line| {
        sb.push_str(&format!(
            "sb.append(\"{}\" + Environment.NewLine);\n",
            line.replace('\r', "")
        ));
    });
    sb.remove(sb.len() - 1);
    sb
}

#[tauri::command]
pub fn revert_stringbuilder(sb: &str) -> String {
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
    text.remove(text.len() - 1);
    text
}
