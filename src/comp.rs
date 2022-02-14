use std::collections::HashMap;

use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
use fermi::{use_set, use_read};

use crate::{ROUTER, CONNECT, RouterState};

#[inline_props]
pub fn TopBar(cx: Scope) -> Element {

    let win = dioxus::desktop::use_window(&cx);

    let set_route = use_set(&cx, ROUTER);
    let connect = use_read(&cx, CONNECT);

    cx.render(rsx!(
        nav {
            
            onmousedown: |_| { win.drag(); },

            class: "navbar is-primary is-fixed-top",
            role: "navigation",

            div {
                class: "navbar-brand",
                a {
                    class: "navbar-item",
                    href: "#",
                    strong { "Dorea Lab" }
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
                            set_route(RouterState {
                                path: "connector".into(),
                                message: String::new(),
                            });
                        },
                        "Connector"
                    }
                    a {
                        class: "navbar-item",
                        onmousedown: |e| { e.cancel_bubble(); },
                        onclick: move |_| {
                            if connect.is_none() {
                                return;
                            }
                            set_route(RouterState {
                                path: "dashboard".to_string(),
                                message: String::new(),
                            });
                        },
                        "Dashboard"
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
        br {}
        br {}
    ))
}