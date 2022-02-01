use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};

#[inline_props]
pub fn TopBar<'a>(cx: Scope, path: &'a UseState<String>) -> Element {

    let win = dioxus::desktop::use_window(&cx);

    cx.render(rsx!(
        nav {
            
            onmousedown: |_| { win.drag(); },

            class: "navbar is-primary",
            role: "navigation",

            div {
                class: "navbar-brand",
                a {
                    class: "navbar-item",
                    href: "#",
                    strong { "DoreaDB" }
                }
                a {
                    class: "navbar-burger",
                    role: "button",
                    "data-target": "navbarMenus",
                    span {}
                    span {}
                    span {}
                }
            }

            div {
                class: "navbar-menu",
                id: "navbarMenus",
                div {
                    class: "navbar-start",
                    a {
                        class: "navbar-item",
                        onmousedown: |e| { e.cancel_bubble(); },
                        onclick: move |_| {
                            path.modify(|_| String::from("dashboard") );
                        },
                        "dashboard",
                    }
                }
                div {
                    class: "navbar-end",
                    a {
                        class: "navbar-item",
                        onmousedown: |e| { e.cancel_bubble(); },
                        onclick: |_| {
                            win.minimize(true);
                        },
                        Icon {
                            icon: Shape::Minus
                        }
                    }
                    a {
                        class: "navbar-item",
                        onmousedown: |e| { e.cancel_bubble(); },
                        onclick: |_| {
                            win.close();
                        },
                        Icon {
                            icon: Shape::X
                        }
                    }
                }
            }

        }
    ))
}