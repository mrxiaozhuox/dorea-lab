use std::{env, fs::{create_dir_all, File}, path::PathBuf, io::{Write, Read}, str::FromStr};

use serde_json::{json, Value};

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

pub fn save_conenct_history(addr: &str, account: (&str, &str)) -> anyhow::Result<()> {
    let current_path = current_dir();
    
    let date = chrono::Local::now().date().to_string();

    let structure = json!(
        {
            "addr": addr,
            "username": account.0,
            "password": account.1,
            "date": date
        }
    );

    if !current_path.join("data").join("connect_history.json").is_file() {
        let mut file = File::create(current_path.join("data").join("connect_history.json"))?;
        let content = json!([
            structure
        ]);
        file.write_all(content.to_string().as_bytes())?;
        return Ok(());
    }

    let mut file = File::open(current_path.join("data").join("connect_history.json"))?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    let mut content = serde_json::from_str::<Vec<Value>>(&buffer)?;

    if content.len() >= 10 {
        content.remove(0);
    }

    content.push(structure);

    file.write_all(Value::Array(content).to_string().as_bytes())?;

    Ok(())
}