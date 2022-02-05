pub use doson::DataValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;

#[derive(Clone, Debug)]
pub struct ClientOption {
    pub addr: String,
    pub account: Account,
    pub main_db: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct RespStruct {
    alpha: String,
    data: Value,
    message: String,
    resptime: u64,
}

pub struct Client {
    config: ClientOption,
    token: String,
    current: String,
}

impl Client {
    pub async fn open(addr: &str, account: Account) -> anyhow::Result<Self> {
        let option = ClientOption {
            addr: addr.to_string(),
            account,
            main_db: String::from("default"),
        };

        Self::open_from_option(option).await
    }

    pub async fn open_from_option(option: ClientOption) -> anyhow::Result<Self> {
        let mut obj = Self {
            config: option.clone(),
            token: Default::default(),
            current: option.main_db,
        };

        if obj.reconnect().await.is_err() {
            return Err(anyhow::anyhow!("account check failed."));
        }

        Ok(obj)
    }

    pub async fn reconnect(&mut self) -> anyhow::Result<()> {
        if &self.config.addr[self.config.addr.len() - 1..] != "/" {
            self.config.addr += "/";
        }

        // 这里需要测试连接的状态（即本次连接是否可以被通过
        let url = self.config.addr.clone() + "auth";

        let params = [
            ("username", &self.config.account.username),
            ("password", &self.config.account.password),
        ];

        let client = reqwest::Client::new();
        let result = client.post(url).form(&params).send().await?;
        let data = result.json::<Value>().await?;

        let v = data.as_object().unwrap();

        let token = v
            .get("data")
            .unwrap()
            .as_object()
            .unwrap()
            .get("token")
            .unwrap()
            .as_str()
            .unwrap();

        self.token = token.to_string();

        Ok(())
    }

    /// change the current databse
    pub fn select(&mut self, db: &str) {
        self.current = String::from(db);
    }

    pub async fn execute(&mut self, command: &str) -> anyhow::Result<String> {
        let url = format!("{}@{}/execute", &self.config.addr, &self.current);
        let client = reqwest::Client::new();
        let res = client
            .post(url)
            .bearer_auth(&self.token)
            .form(&[("query", command)])
            .send()
            .await?;

        let resp = res.json::<RespStruct>().await.unwrap_or(RespStruct {
            alpha: "ERR".to_string(),
            data: Value::Null,
            message: String::new(),
            resptime: 0,
        });

        if &resp.alpha != "OK" {
            return Err(anyhow::anyhow!(resp.message.clone()));
        }

        let data = resp
            .data
            .as_object()
            .unwrap()
            .get("reply")
            .unwrap()
            .as_str()
            .unwrap();

        return Ok(data.to_string());
    }

    pub async fn get(&mut self, key: &str) -> doson::DataValue {
        let v = self.execute(&format!("get {}", key)).await;
        if v.is_err() {
            return DataValue::None;
        }
        return DataValue::from(&v.unwrap());
    }

    pub async fn setex(
        &mut self,
        key: &str,
        value: DataValue,
        expire: usize,
    ) -> anyhow::Result<()> {
        let command = format!("set {} {} {}", key, value.to_string(), expire);

        let res = self.execute(&command).await?;

        if res == "" {
            return Ok(());
        } else {
            return Err(anyhow::anyhow!(res.clone()));
        }
    }

    pub async fn set(&mut self, key: &str, value: DataValue) -> anyhow::Result<()> {
        self.setex(key, value, 0).await
    }

    pub async fn delete(&mut self, key: &str) -> bool {
        let res = self.execute(&format!("delete {}", key)).await;
        res.is_ok()
    }

    pub async fn clean(&mut self) -> bool {
        let res = self.execute("clean").await;
        res.is_ok()
    }
}

pub struct BlockingClient {
    inner: Client,
    rt: Runtime,
}

impl BlockingClient {
    pub fn open(addr: &str, account: Account) -> anyhow::Result<Self> {
        let option = ClientOption {
            addr: addr.to_string(),
            account,
            main_db: String::from("default"),
        };

        Self::open_from_option(option)
    }
    
    pub fn open_from_option(option: ClientOption) -> anyhow::Result<Self> {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        let inner = rt.block_on(Client::open_from_option(option))?;
        Ok(Self { inner, rt })
    }

    pub fn execute(&mut self, command: &str) -> anyhow::Result<String> {
        self.rt.block_on(self.inner.execute(command))
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
