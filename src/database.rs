use std::collections::HashMap;

use dorea_wsc::{Account, Client};
use serde::{Deserialize, Serialize};

pub async fn try_connect(addr: &str, account: (&str, &str)) -> anyhow::Result<()> {
    let account = Account::new(account.0.to_string(), account.1.to_string());
    let mut c = Client::open(addr, account).await?;
    c.execute("ping").await?;
    Ok(())
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseInfo {
    pub name: String,
    pub index_number: usize,
    pub database_state: String,
    pub account_state: bool,
    pub weight: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
struct DatabaseStatusMeta {
    index_num: usize,
    state: String,
    weight: u32,
}

pub async fn db_list_info(
    mut client: Client,
    usable_list: Option<Vec<String>>,
) -> Vec<DatabaseInfo> {
    let list = client
        .execute("db status")
        .await
        .unwrap_or_else(|_| String::from("[]"));
    let meta = serde_json::from_str::<HashMap<String, DatabaseStatusMeta>>(&list).unwrap();

    let mut result = vec![];
    for (key, item) in meta {
        let account_state =
            usable_list.is_none() || usable_list.clone().unwrap_or_default().contains(&key);
        result.push(DatabaseInfo {
            name: key.clone(),
            index_number: item.index_num,
            database_state: item.state,
            account_state,
            weight: item.weight,
        });
    }

    result
}

pub async fn unlock_db(mut client: Client, name: &str) -> anyhow::Result<()> {
    let res = client.execute(&format!("db unlock {}", name)).await?;
    if &res == "Successful." {
        return Ok(())
    }
    Err(anyhow::anyhow!(res))
}