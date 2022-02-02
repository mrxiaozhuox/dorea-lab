use dorea_wsc::{Account, Client};

fn main() {
    let c = Client::open(
        "http://127.0.0.1:3451/".into(),
        Account::new("master".into(), "DOREA@SERVICE".into()),
    ).unwrap();
}
