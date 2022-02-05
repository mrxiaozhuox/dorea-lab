use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Values {
    pub inner: Items,
}

impl Display for Values {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(Debug, Clone)]
pub enum Items {
    ConnectButton,
}

impl Display for Items {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Items::ConnectButton => "连接",
        };
        write!(f, "{}", c)
    }
}