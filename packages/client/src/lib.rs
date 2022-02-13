pub use doson::DataValue;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::runtime::Runtime;

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct Client {
    config: ClientOption,
    token: String,
    pub current: String,
    usa_db: Option<Vec<String>>
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
            usa_db: None,
        };

        if &obj.config.addr[obj.config.addr.len() - 1..] != "/" {
            obj.config.addr += "/";
        }

        // if let Err(e) = obj.reconnect().await {
        //     return Err(anyhow::anyhow!("account check failed."));
        // }

        obj.reconnect().await?;

        Ok(obj)
    }

    pub fn addr(&self) -> String {
        self.config.addr.to_string()
    }

    pub async fn reconnect(&mut self) -> anyhow::Result<()> {
        // 这里需要测试连接的状态（即本次连接是否可以被通过
        let url = self.config.addr.clone() + "auth";

        let params = [
            ("username", &self.config.account.username),
            ("password", &self.config.account.password),
        ];

        let client = reqwest::Client::new();
        let result = client.post(url).form(&params).send().await?;
        let data = result.json::<RespStruct>().await?;

        // let v = data.as_object().unwrap();

        // 接口请求失败
        if data.alpha != "OK" {
            return Err(anyhow::anyhow!(data.message));
        }

        let values = data.data.as_object().unwrap();
        let token = values.get("token").unwrap().as_str().unwrap();

        // println!("{:?}", values);

        let username = self.config.account.username.clone();

        // 这里会检测数据库允许使用的列表，如果 Default 并未在行列之中，则随机选择一个作为默认连接目标。
        let usa_db = values.get("usa_db").unwrap().as_array().unwrap();
        let default_db = if usa_db.contains(&Value::String("default".into())) {
            String::from("default")
        } else if usa_db.contains(&Value::String(username.clone())) {
            String::from(&username)
        } else {
            if usa_db.len() >= 1 {
                String::from(usa_db.get(usa_db.len() - 1).unwrap().as_str().unwrap())
            } else {
                String::from("default")
            }
        };

        // 如果当前库并不被支持，则自动将其转换为 default
        if self.current != default_db
            && !usa_db.contains(&Value::String(self.current.clone()))
            && usa_db.len() >= 1
        {
            self.current = default_db;
        }

        let temp = serde_json::Value::Array(usa_db.clone()).to_string();
        let temp = serde_json::from_str::<Vec<String>>(&temp);
        if let Ok(v) = temp {
            if v.len() > 0 {
                self.usa_db = Some(v.clone());
            }
        }

        // println!("current: {}", self.current);

        // let token = v
        //     .get("data")
        //     .unwrap()
        //     .as_object()
        //     .unwrap()
        //     .get("token")
        //     .unwrap()
        //     .as_str()
        //     .unwrap();

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

#[derive(Clone, Debug, PartialEq)]
pub struct Account {
    pub username: String,
    pub password: String,
}

impl Account {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}
