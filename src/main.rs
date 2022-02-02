#![allow(non_snake_case)]

mod comp;
mod database;

use comp::*;
use dioxus::prelude::*;
use fermi::{Atom, use_read, use_set};

struct RouterState {
    path: String
}

static ROUTER: Atom<RouterState> = |_| {
    RouterState {
        path: String::from("dashboard")
    }
};

fn main() {
    use dioxus::desktop::tao::dpi::LogicalSize;
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| {
            w.with_title("Dioxus Lab | ⛺")
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(1050.0, 650.0))
                .with_decorations(false)
        })
    });
}

fn app(cx: Scope) -> Element {

    let router = use_read(&cx, ROUTER);

    let body = match router.path.as_str() {
        "dashboard" => Dashboard(cx),
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

fn Dashboard<'a>(cx: Scope<'a>) -> Element {

    let tcp_conn = use_state(&cx, || true);
    let hostname_state = use_state(&cx, || "127.0.0.1".to_string());
    let port_state = use_state(&cx, || 3450_u16);
    let password_state = use_state(&cx, || "".to_string());

    cx.render(rsx!(
        div {
            class: "card",
            div {
                class: "card-content",
                div {
                    class: "columns",
                    div {
                        class: "column is-2",
                        div {
                            class: "select is-fullwidth",
                            select {
                                if *tcp_conn.0 {
                                    cx.render(rsx!(
                                        option { selected: "true", "TCP" },
                                        option { "HTTP" }
                                    ))
                                } else {
                                    cx.render(rsx!(
                                        option { "TCP" },
                                        option { selected: "true", "HTTP" }
                                    ))
                                }
                            }
                        }
                    }
                    div {
                        class: "column is-3",
                        input {
                            class: "input is-info",
                            r#type: "text",
                            placeholder: "Hostname",
                            value: "{hostname_state.0}",
                            oninput: move |v| {
                                hostname_state.1.modify(|_| v.value.clone());
                            },
                            style: "font-weight:bold;",
                        }
                    }
                    div {
                        class: "column is-2",
                        input {
                            class: "input is-info",
                            r#type: "number",
                            placeholder: "Port",
                            value: "{port_state.0}",
                            oninput: move |v| {
                                port_state.1.modify(|_| v.value.parse::<u16>().unwrap_or(3450));
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
                                
                                let addr = (
                                    hostname_state.0.to_string(),
                                    *port_state.0
                                );
                                let password = password_state.0.clone();
                                let set_route = use_set(&cx, ROUTER).clone();
                            },
                            "Connect"
                        }
                    }
                }
            }
        }
    ))
}
