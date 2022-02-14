use dioxus::prelude::*;
use fermi::use_read;

use crate::CONNECT;

#[inline_props]
pub fn Information(cx: Scope) -> Element {
    let connect = use_read(&cx, CONNECT).clone().unwrap();

    let version = use_state(&cx, String::new);
    let startup_time = use_state(&cx, String::new);

    let version_setter = version.1.clone();
    let startup_timme_setter = version.1.clone();
    cx.spawn(async move {
        let mut client = connect.client.clone();
        let v = client
            .execute("info version")
            .await
            .unwrap_or_else(|_| String::from("Unknown"));
        version_setter(v);
        let v = client.execute("info server-startup-time").await.unwrap_or_else(|_| String::from("Unknown"));
        println!("{:?}", v);
    });

    cx.render(rsx!(
        div {
            class: "level",
            div {
                class: "level-item has-text-centered",
                div {
                    p {
                        class: "heading",
                        "Dorea Version"
                    }
                    p {
                        class: "title",
                        "{version.0}"
                    }
                }
            }
            div {
                class: "level-item hash-text-centered",
                div {
                    p {
                        class: "heading",
                        "Startup Time"
                    },
                    p {
                        class: "title",
                        "{startup_time.0}"
                    }
                }
            }
        }
    ))
}
