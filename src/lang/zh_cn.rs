#[allow(clippy::match_str_case_mismatch)]
pub fn load(name: &str) -> String {
    let v = match name.to_lowercase().as_str() {
        "connect" => "连接",

        "connector" => "连接管理",
        "connector:connect_prompt_message" => "请将用于 `Web-Serivce` 连接的信息填写至此。【URL、用户名、密码】",
        
        "failed:connect_error" => "数据库连接错误：",
        
        _ => "__UNKNOWN__",
    };
    v.to_string()
}