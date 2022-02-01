#![allow(non_snake_case)]

mod comp;

use dioxus::prelude::*;
use comp::*;
use dioxus_heroicons::Icon;

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

    let inner_path = use_state(&cx, || String::from("dashboard"));
    
    let body = match inner_path.0.as_str() {
        "dashboard" => {
            Dashboard(cx)
        },
        _ => {
            cx.render(rsx!(
                h1 { "404 Not found" }
            ))
        }
    };

    let path_state = inner_path.1;

    cx.render(rsx! (
        style { [ include_str!("./assets/bulma.min.css") ] }
        // hidden the scroll bar
        style { "html {{overflow-x: hidden; overflow-y: hidden;}}" }

        TopBar {
            path: path_state
        }
        br {}

        div {
            class: "container",
            body
        }

        script { [ include_str!("./assets/app.js") ] }
    ))
}


fn Dashboard(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "card",
            div {
                class: "card-content",
                div {
                    class: "columns",
                    div {
                        class: "column is-10",
                        input {
                            class: "input is-info",
                            r#type: "text",
                            placeholder: "Input Dorea-Server [TCP]: dioxus://127.0.0.1:8090/",
                            style: "font-weight:bold;"
                        }
                    }
                    div {
                        class: "column is-2",
                        button {
                            class: "button is-fullwidth is-info",
                            "Connect"
                        }
                    }
                }
            }
        }
    ))
}