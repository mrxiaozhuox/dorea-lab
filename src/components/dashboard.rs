use chrono::TimeZone;
use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};
use fermi::use_read;

use crate::CONNECT;

#[inline_props]
pub fn Information(cx: Scope) -> Element {
    let connect = use_read(&cx, CONNECT).clone().unwrap();

    let version = use_state(&cx, String::new);
    let startup_time = use_state(&cx, String::new);
    let loaded_db_num = use_state(&cx, String::new);
    let connect_id = use_state(&cx, String::new);
    let loaded_db = use_state(&cx, Vec::<String>::new);

    let current_user = connect.account.username;

    let usa_db = connect.client.usa_db.clone();
    let loaded_db_list = loaded_db.0.iter().map(|v| {
        let icon = if usa_db.is_none() {
            rsx! {
                Icon {
                    icon: Shape::ShieldCheck,
                    fill: "green",
                }
            }
        } else {
            let t = usa_db.as_ref().unwrap();
            if t.contains(v) {
                rsx! {
                    Icon {
                        icon: Shape::ShieldCheck,
                        fill: "green",
                    }
                }
            } else {
                rsx! {
                    Icon {
                        icon: Shape::ShieldExclamation,
                        fill: "red",
                    }
                }
            }
        };

        rsx! (
           a {
                class: "panel-block is-warn",
                span {
                    style: "float: right",
                    class: "panel-icon",
                    icon
                }
                strong { "{v}" }
           }
        )
    });

    // 这里主要做所有数据的初始化，一些需要实时定期被更新的数据会在其他任务中被处理
    if version.0.is_empty() {
        let version_setter = version.1.clone();
        let startup_timme_setter = startup_time.1.clone();
        let loaded_db_num_setter = loaded_db_num.1.clone();
        let connect_id_setter = connect_id.1.clone();
        let loaded_db_setter = loaded_db.1.clone();
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
            let startup_date = chrono::Utc.timestamp(temp.parse().unwrap_or(0), 0);

            let loaded_num = client
                .execute("db num")
                .await
                .unwrap_or_else(|_| String::from("Unknown"));

            let connect_id = client
                .execute("info cid")
                .await
                .unwrap_or_else(|_| String::from("Unknown"));

            let temp = client
                .execute("db list")
                .await
                .unwrap_or_else(|_| String::from("Unknown"));
            let temp = doson::DataValue::from(&temp);
            let temp = temp.as_list().unwrap_or_default();
            let mut loaded_db = vec![];
            for item in temp {
                loaded_db.push(item.as_string().unwrap_or_default());
            }

            version_setter(version);
            startup_timme_setter(startup_date.date().format("%Y-%m-%d").to_string());
            loaded_db_num_setter(loaded_num);
            connect_id_setter(connect_id);
            loaded_db_setter(loaded_db);
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
                        "Current User"
                    }
                    p {
                        class: "title",
                        "{current_user}"
                    }
                }
            }
            div {
                class: "level-item has-text-centered",
                div {
                    p {
                        class: "heading",
                        "Server Startup Time"
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
                        "Loaded Database Num"
                    }
                    p {
                        class: "title",
                        "{loaded_db_num.0}"
                    }
                }
            }
        }
        hr {}
        div {
            class: "field has-addons",
            p {
                class: "control",
                a {
                    class: "button is-static",
                    "Connect ID"
                }
            }
            p {
                class: "control is-expanded",
                input {
                    class: "input is-primary",
                    r#type: "text",
                    value: "{connect_id.0}",
                    disabled: "true",
                }
            }
        }
        hr {}
        nav {
            class: "panel",
            p {
                class: "panel-heading",
                "Loaded Database"
            }
            loaded_db_list
        }
    ))
}

#[inline_props]
pub fn Databses(cx: Scope) -> Element {
    let connect = use_read(&cx, CONNECT).clone().unwrap();

    let usable_db_list = connect.client.usa_db.clone();
    let loaded_db_list = use_state(&cx, Vec::<String>::new);

    if loaded_db_list.0.is_empty() {
        let loaded_db_setter = loaded_db_list.1.clone();
        cx.spawn(async move {
            let mut client = connect.client.clone();
            let v = client
                .execute("db list")
                .await
                .unwrap_or_else(|_| String::from("[]"));
        });
    }

    cx.render(rsx! {
        table {
            class: "table is-bordered is-hoverable is-striped is-fullwidth",
            thead {
                tr {
                    th { "Name" }
                    th { "Index Number" }
                    th { "Database State" }
                    th { "Account State" }
                    th { "Operation" }
                }
            }
            tbody {  }
        }
    })
}
