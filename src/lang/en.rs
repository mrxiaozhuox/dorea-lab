#[allow(clippy::match_str_case_mismatch)]
pub fn load(name: &str) -> String {
    let v = match name.to_lowercase().as_str() {
        "connect" => "connect",

        "connector" => "connector",
        "connector:connect_prompt_message" => "Please fill ` Web-Serice ` connect information here.",

        "failed:connect_error" => "Database connect failed: ",

        _ => "__UNKNOWN__",
    };
    v.to_string()
}