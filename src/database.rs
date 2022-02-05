use dorea_wsc::{Account, Client};

pub async fn try_connect(addr: &str, account: (&str, &str)) -> bool {
    let account = Account::new(account.0.to_string(), account.1.to_string());

    if let Ok(mut c) = Client::open(addr, account).await {
        if let Ok(value) = c.execute("ping").await {
            return value.to_uppercase() == "PONG";
        }
    }
    false
}