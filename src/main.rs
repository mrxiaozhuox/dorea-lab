#![allow(non_snake_case)]

mod comp;
mod database;
mod lang;

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
}

struct ConnectState {
    account: Account,
    client: Client,
}

static ROUTER: Atom<RouterState> = |_| RouterState {
    path: String::from("connector"),
};

static CONNECT: Atom<Option<ConnectState>> = |_| None;

const DEFAULT_LANGUAGE: &str = "zh_cn";

fn main() {
    use dioxus::desktop::tao::dpi::LogicalSize;
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

    let lang = DEFAULT_LANGUAGE.to_string();

    cx.use_hook(|_| { cx.provide_context(LangShared { lang }) });

    let body = match router.path.as_str() {
        "connector" => Connector(cx),
        _ => cx.render(rsx!(
            h1 { "404 Not found" }
        )),
    };

    cx.render(rsx! (
        style { [ include_str!("./assets/bulma.min.css") ] }
        // hidden the scroll bar
        style { "html {{overflow-x: hidden; overflow-y: hidden;}}" }

        TopBar {}
        br {}

        div {
            class: "container",
            body
        }

        script { [ include_str!("./assets/app.js") ] }
    ))
}

fn Connector(cx: Scope) -> Element {
    let addr_state = use_state(&cx, || "http://127.0.0.1:3451/".to_string());
    let username_state = use_state(&cx, || "master".to_string());
    let password_state = use_state(&cx, || "".to_string());

    let lang = cx.consume_context::<LangShared>().unwrap();
    let lang = &lang.lang;

    cx.render(rsx!(
        div {
            class: "card",
            div {
                class: "card-content",
                article {
                    class: "message is-info",
                    div {
                        class: "message-body",
                        [ load_text(lang, "connector:connect_prompt_message") ]
                    }
                }
                // br {}
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
                            onclick: move |_| {

                                let addr = addr_state.0.clone();
                                let username = username_state.0.clone();
                                let password = password_state.0.clone();

                                let set_route = use_set(&cx, ROUTER).clone();
                                let set_connect = use_set(&cx, CONNECT).clone();

                                cx.spawn(async move {
                                    if database::try_connect(&addr, (&username, &password)).await {

                                        let account = Account::new(username.clone(), password.clone());

                                        set_connect(Some(ConnectState {
                                            client: Client::open(&addr, account.clone()).await.unwrap(),
                                            account,
                                        }));

                                        // 跳转到 管理页面
                                        set_route(RouterState {
                                            path: "dashboard".to_string(),
                                        });
                                    }
                                })
                            },
                            [ crate::lang::load(lang, "connect") ]
                        }
                    }
                }
            }
        }
    ))
}
