use std::fmt::Display;

pub mod zh_cn;
pub mod en;

pub fn lang<T: Display>(lang: &'static str) -> T {
    zh_cn::Items
}