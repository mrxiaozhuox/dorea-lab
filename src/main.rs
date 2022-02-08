#![allow(non_snake_case)]

mod comp;
mod database;
mod lang;
mod storage;

use std::process::exit;

use comp::*;
use dioxus::prelude::*;
use dorea_wsc::{Account, Client};
use fermi::{use_read, use_set, Atom};

use crate::lang::load as load_text;

struct LangShared {
    lang: String,
}

struct RouterState {
    path: String,
    message: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ConnectState {
    account: Account,
    client: Client,
}

static ROUTER: Atom<RouterState> = |_| RouterState {
    path: String::from("connector"),
    message: String::new(),
};

static CONNECT: Atom<Option<ConnectState>> = |_| None;

const DEFAULT_LANGUAGE: &str = "zh_cn";

fn main() {
    use dioxus::desktop::tao::dpi::LogicalSize;
    crate::storage::init_dir().unwrap();
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| {
            w.with_title("Dorea Lab | ⛺")
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(1050.0, 650.0))
                .with_decorations(false)
        })
    });
}

fn app(cx: Scope) -> Element {

    let router = use_read(&cx, ROUTER);
    let client = use_read(&cx, CONNECT).clone();

    let lang = DEFAULT_LANGUAGE.to_string();

    cx.use_hook(|_| { cx.provide_context(LangShared { lang }) });

    let body = match router.path.as_str() {
        "connector" => rsx!( Connector {} ),
        "dashboard" => {
            rsx!( Dashboard { client: client.unwrap() } )
        },
        "failed" => rsx!(Failed { message: router.message.clone() }),
        _ => rsx!(
            h1 { "404 Not found" }
        ),
    };

    cx.render(rsx! (
        style { [ include_str!("./assets/bulma.min.css") ] }
        // hidden the scroll bar
        style { "html::-webkit-scrollbar {{display: none;}}" }

        TopBar {}
        br {}

        div {
            class: "container",
            body
        }

        script { [ include_str!("./assets/app.js") ] }
    ))
}

#[inline_props]
fn Dashboard(cx: Scope, client: ConnectState) -> Element {
    
    let lang_shared = cx.consume_context::<LangShared>().unwrap();
    let lang = lang_shared.lang.clone();

    let addr = client.client.addr();

    let (username, password) = (&client.account.username, &client.account.password);

    storage::save_conenct_history(&addr, (username, password)).unwrap();    

    cx.render(rsx!(
        div {
            "123",
        }
    ))
}

#[inline_props]
fn Connector(cx: Scope) -> Element {
    let addr_state = use_state(&cx, || "http://127.0.0.1:3451/".to_string());
    let username_state = use_state(&cx, || "master".to_string());
    let password_state = use_state(&cx, || "".to_string());

    let lang_shared = cx.consume_context::<LangShared>().unwrap();
    let lang = lang_shared.lang.clone();

    // 这里用于更新 Connector 消息页面的内容
    let (message, message_setter) = use_state(&cx, || {
        ("info".to_string(), load_text(&lang, "connector:connect_prompt_message"))
    });

    let btn_disabled = use_state(&cx, || "false".to_string());


    let connect_historys = storage::load_connect_history();
    let connect_historys = connect_historys.iter().map(move |v| {
        let str = format!("{} [ {} ] | {}", v.addr, v.username, v.date);

        let conn_info = v.clone();
        let set_addr = addr_state.1.clone();
        let set_username = username_state.1.clone();
        let set_password = password_state.1.clone();

        rsx! {
            li {
                a {
                    href: "#",
                    onclick: move |_| {
                        set_addr(conn_info.addr.clone());
                        set_username(conn_info.username.clone());
                        set_password(conn_info.password.clone());
                    },
                    strong { "{str}" }
                }
            }
        }
    });

    cx.render(rsx!(
        div {
            class: "card",
            div {
                class: "card-content",
                article {
                    class: "message is-{message.0}",
                    div {
                        class: "message-body",
                        "{message.1}",
                    }
                }
                div {
                    class: "columns",
                    div {
                        class: "column is-4",
                        input {
                            class: "input is-info",
                            r#type: "text",
                            placeholder: "Hostname",
                            value: "{addr_state.0}",
                            oninput: move |v| {
                                addr_state.1.modify(|_| v.value.clone());
                            },
                            style: "font-weight:bold;",
                        }
                    }
                    div {
                        class: "column is-3",
                        input {
                            class: "input is-info",
                            r#type: "text",
                            placeholder: "Username",
                            value: "{username_state.0}",
                            oninput: move |v| {
                                username_state.1.modify(|_| v.value.clone());
                            },
                            style: "font-weight:bold;",
                        }
                    }
                    div {
                        class: "column is-3",
                        input {
                            class: "input is-info",
                            r#type: "password",
                            placeholder: "Password",
                            value: "{password_state.0}",
                            oninput: move |v| {
                                password_state.1.modify(|_| v.value.clone());
                            },
                            style: "font-weight:bold;",
                        }
                    }
                    div {
                        class: "column is-2",
                        button {
                            class: "button is-fullwidth is-info",
                            disabled: "{btn_disabled.0}",
                            onclick: move |_| {

                                let addr = addr_state.0.clone();
                                let username = username_state.0.clone();
                                let password = password_state.0.clone();

                                let set_route = use_set(&cx, ROUTER).clone();
                                let set_connect = use_set(&cx, CONNECT).clone();

                                let lang_info = lang_shared.lang.clone();
                                
                                let message_setter = message_setter.clone();

                                let btn_disabled_setter = btn_disabled.1.clone();

                                btn_disabled_setter("true".into());

                                cx.spawn(async move {
                                    // println!("请等待连接！");
                                    match database::try_connect(&addr, (&username, &password)).await {
                                        Ok(_) => {
                                            let account = Account::new(username.clone(), password.clone());

                                            set_connect(Some(ConnectState {
                                                client: Client::open(&addr, account.clone()).await.unwrap(),
                                                account,
                                            }));
    
                                            // 跳转到 管理页面
                                            set_route(RouterState {
                                                path: "dashboard".to_string(),
                                                message: String::new(),
                                            });
                                        }
                                        Err(e) => {
                                            // 这里会更换 message 为 error 并显示连接错误的内容
                                            let message = format!(
                                                "{}[ {} ]", 
                                                load_text(&lang_info, "failed:connect_error"),
                                                e
                                            );
                                            message_setter(("danger".into(), message));
                                        }
                                    }
                                    btn_disabled_setter("false".into());
                                })
                            },
                            [ crate::lang::load(&lang, "connect") ]
                        }
                    }
                }
            }
        }

        br {}

        div {
            class: "card",
            div {
                class: "card-content",
                p {
                    class: "subtitle is-5",
                    [ load_text(&lang, "connector:connect_history") ]
                }
                style { ["#historys li { margin-bottom: 15px }"] }
                ul {
                    id: "historys",
                    connect_historys
                }
            }
        }

    ))
}

#[inline_props]
fn Failed(cx: Scope, message: String) -> Element {
    cx.render(rsx!(
        div {
            dangerous_inner_html: "{message}",
        }
    ))
}