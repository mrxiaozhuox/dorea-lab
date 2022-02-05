use dioxus::prelude::*;
use dioxus_heroicons::{Icon, solid::Shape};
use fermi::use_set;

use crate::{ROUTER, RouterState};

#[inline_props]
pub fn TopBar(cx: Scope) -> Element {

    let win = dioxus::desktop::use_window(&cx);

    let set_route = use_set(&cx, ROUTER);

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
                                path: "dashboard".into()
                            });
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