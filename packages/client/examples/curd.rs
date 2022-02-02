use dorea_wsc::{Account, Client, DataValue};

fn main() {
    let mut c = Client::open(
        "http://127.0.0.1:3451/".into(),
        Account::new("master".into(), "DOREA@SERVICE".into()),
    ).unwrap();

    c.setex("foo", DataValue::Number(10.0), 0).unwrap();

    let res = c.get("foo");

    println!("{:?}", res);

}