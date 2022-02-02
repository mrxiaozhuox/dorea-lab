use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ClientOption {
    pub addr: String,
    pub account: Account,
    pub main_db: String,
}

pub struct Client {
    config: ClientOption,
}

impl Client {
    pub fn open(addr: String, account: Account) -> anyhow::Result<Self> {
        let option = ClientOption {
            addr,
            account,
            main_db: String::from("default"),
        };

        Self::open_from_option(option)
    }

    pub fn open_from_option(mut option: ClientOption) -> anyhow::Result<Self> {
        if &option.addr[option.addr.len() - 1..] != "/" {
            option.addr += "/";
        }

        // 这里需要测试连接的状态（即本次连接是否可以被通过
        let url = option.addr.clone() + "auth";

        let runtime = tokio::runtime::Runtime::new()?;

        let params = [
            ("username", &option.account.username),
            ("password", &option.account.password),
        ];

        let client = reqwest::Client::new();
        let result = runtime.block_on(client.post(url).form(&params).send())?;
        println!("{:?}", runtime.block_on(result.text()));
        Ok(Self { config: option })
    }
}

#[derive(Clone, Debug)]
pub struct Account {
    username: String,
    password: String,
}

impl Account {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}
