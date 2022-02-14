use chrono::TimeZone;
use dioxus::prelude::*;
use fermi::use_read;

use crate::CONNECT;

#[inline_props]
pub fn Information(cx: Scope) -> Element {
    let connect = use_read(&cx, CONNECT).clone().unwrap();

    let version = use_state(&cx, String::new);
    let startup_time = use_state(&cx, String::new);

    if version.0.is_empty() {
        let version_setter = version.1.clone();
        let startup_timme_setter = startup_time.1.clone();
        cx.spawn(async move {
            let mut client = connect.client.clone();
            let version = client
                .execute("info version")
                .await
                .unwrap_or_else(|_| String::from("Unknown"));
    
            let temp = client
                .execute("info server-startup-time")
                .await
                .unwrap_or_else(|_| String::from("Unknown"));
            let dt = chrono::Utc.timestamp(temp.parse().unwrap_or(0), 0);
    
            version_setter(version);
            startup_timme_setter(dt.date().format("%Y-%m-%d").to_string());
        });
    }

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
                class: "level-item has-text-centered",
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
            div {
                class: "level-item has-text-centered",
                div {
                    p {
                        class: "heading",
                        "Loaded Database"
                    }
                    p {
                        class: "title",
                        "50"
                    }
                }
            }
        }
    ))
}
