pub(super) fn msg_json(message: Option<&str>) -> String {
    message.map_or(String::new(), |m| {
        format!(
            r#","message":{}"#,
            serde_json::to_string(m).unwrap_or_else(|_| "null".into())
        )
    })
}

pub(super) fn emit_json(items: Vec<String>) {
    println!(
        "{}",
        if items.is_empty() {
            "[]".into()
        } else {
            format!("[\n{}\n]", items.join(",\n"))
        }
    );
}
