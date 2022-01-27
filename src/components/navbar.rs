use dioxus::prelude::*;

#[inline_props]
pub fn Navbar<'a>(cx: Scope<'a>, path: UseState<'a, String>) -> Element {
    cx.render(rsx!(
        nav {
            
            class: "navbar is-light",
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
                        onclick: move |_| {
                            path.set("dashboard".into());
                        },
                        "dashboard",
                    }

                }
            }

        }
    ))
}