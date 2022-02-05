pub mod en;
pub mod zh_cn;

pub fn load(lang: &str, name: &str) -> String {
    if lang == "en" {
        let temp = en::load(name);
        if &temp != "__UNKNOWN__" {
            return temp;
        }
    }
    zh_cn::load(name)
}
