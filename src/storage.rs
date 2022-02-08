use std::{
    env,
    fs::{create_dir_all, File},
    io::{Read, Write},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

fn current_dir() -> PathBuf {
    env::current_exe()
        .unwrap()
        .parent()
        .ok_or(anyhow::anyhow!("dir not found"))
        .unwrap()
        .to_path_buf()
}

pub fn init_dir() -> anyhow::Result<()> {
    let current_path = current_dir();

    if current_path.join("data").is_dir() {
        return Ok(());
    }
    create_dir_all(current_path.join("data"))?;
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectHistoryInfo {
    pub addr: String,
    pub username: String,
    pub password: String,
    pub date: String,
}

pub fn save_conenct_history(addr: &str, account: (&str, &str)) -> anyhow::Result<()> {
    let current_path = current_dir();

    let date = chrono::Local::now().date().to_string();

    let structure = ConnectHistoryInfo {
        addr: addr.to_string(),
        username: account.0.to_string(),
        password: account.1.to_string(),
        date,
    };

    let mut content = vec![];
    if current_path
        .join("data")
        .join("connect-history.json")
        .is_file()
    {
        let mut file = File::open(current_path.join("data").join("connect-history.json"))?;
        let mut buffer = String::new();
        file.read_to_string(&mut buffer)?;

        content = serde_json::from_str::<Vec<ConnectHistoryInfo>>(&buffer)?;
    }

    if content.len() >= 5 {
        content.remove(0);
    }

    content.push(structure);

    let mut file = File::create(current_path.join("data").join("connect-history.json"))?;
    file.write_all(
        serde_json::to_string(&content)
            .unwrap_or_else(|_| "[]".to_string())
            .as_bytes(),
    )?;

    Ok(())
}

pub fn load_connect_history() -> Vec<ConnectHistoryInfo> {
    let current_path = current_dir();

    if !current_path
        .join("data")
        .join("connect-history.json")
        .is_file()
    {
        return vec![];
    }

    let file = File::open(current_path.join("data").join("connect-history.json"));
    if file.is_err() {
        return vec![];
    }
    let mut file = file.unwrap();

    let mut buffer = String::new();
    if file.read_to_string(&mut buffer).is_err() {
        return vec![];
    }

    serde_json::from_str::<Vec<ConnectHistoryInfo>>(&buffer).unwrap_or_default()
}