#[allow(clippy::match_str_case_mismatch)]
pub fn load(name: &str) -> String {
    println!("SB: {}", name);
    let v = match name.to_lowercase().as_str() {
        "connect" => "连接",
        "connector:connect_prompt_message" => "请将用于 `Web-Serivce` 连接的信息填写至此。【URL、用户名、密码】",
        _ => "__UNKNOWN__",
    };
    v.to_string()
}