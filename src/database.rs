use dorea_wsc::{Account, Client};

pub async fn try_connect(addr: &str, account: (&str, &str)) -> anyhow::Result<()> {
    let account = Account::new(account.0.to_string(), account.1.to_string());
    let mut c = Client::open(addr, account).await?;
    c.execute("ping").await?;
    Ok(())
}
