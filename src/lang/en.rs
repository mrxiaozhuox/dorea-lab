#[allow(clippy::match_str_case_mismatch)]
pub fn load(name: &str) -> String {
    println!("SB: {}", name);
    let v = match name.to_lowercase().as_str() {
        "connect" => "connect",
        "connector:connect_prompt_message" => "Please fill ` Web-Serice ` connect information here.",
        _ => "__UNKNOWN__",
    };
    v.to_string()
}