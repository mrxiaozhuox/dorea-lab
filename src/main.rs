#![allow(non_snake_case)]

mod components;

use std::process::exit;

use components::*;
use dioxus::prelude::*;

fn main() {
    use dioxus::desktop::tao::dpi::LogicalSize;
    dioxus::desktop::launch_cfg(app, |cfg| {
        cfg.with_window(|w| {
            w.with_title("Dioxus Lab | ⛺")
                .with_resizable(true)
                .with_inner_size(LogicalSize::new(1050.0, 650.0))
        })
    });
}

fn app(cx: Scope) -> Element {

    let inner_path = use_state(&cx, || String::from("dashboard"));
    
    let body = match inner_path.get().as_str() {
        "dashboard" => {
            Dashboard(cx)
        },
        _ => None
    };

    cx.render(rsx! (
        style { [ include_str!("./assets/bulma.min.css") ] }
        // hidden the scroll bar
        style { "html {{overflow-x: hidden; overflow-y: hidden;}}" }

        navbar::Navbar {
            path: inner_path
        }

        body

        script { [ include_str!("./assets/app.js") ] }
    ))
}


fn Dashboard(cx: Scope) -> Element {
    cx.render(rsx!(
        h1 { "Dashboard" }
    ))
}