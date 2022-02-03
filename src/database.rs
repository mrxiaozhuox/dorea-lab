use dorea_wsc::{Account, Client};

pub fn try_connect(addr: &str, account: (&str, &str)) -> bool {
    let account = Account::new(account.0.to_string(), account.1.to_string());

    if let Ok(mut c) = Client::open(addr, account) {
        if let Ok(value) = c.execute("ping") {
            return value.to_uppercase() == "PONG";
        }
    }
    false
}