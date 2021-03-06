use chrono::TimeZone;
use dioxus::prelude::*;
use dioxus_heroicons::{solid::Shape, Icon};
use fermi::use_read;

use crate::{
    database::{self, db_list_info, DatabaseInfo},
    CONNECT,
};

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

    let need_reload = use_state(&cx, || true);

    let usable_db_list = connect.client.usa_db.clone();
    let display_list = use_state(&cx, Vec::<DatabaseInfo>::new);

    let display_connect = connect.clone();
    let need_reload_setter = need_reload.1.clone();
    let display_rsx = display_list.0.iter().map(|value| {
        let unload_title = if value.database_state == "Locked" {
            "Unlock"
        } else {
            "Lock"
        };

        let connect = display_connect.clone();

        let need_reload_setter = need_reload_setter.clone();

        rsx! {
            tr {
                th {
                    "{value.name}"
                }
                td {
                    "{value.index_number}"
                }
                td {
                    "{value.database_state}"
                }
                td {
                    if value.account_state {
                        rsx! {
                            Icon {
                                icon: Shape::ShieldCheck,
                                size: 14,
                                fill: "green",
                            }
                            strong { "Authorised" }
                        }
                    } else {
                        rsx! {
                            Icon {
                                icon: Shape::ShieldExclamation,
                                size: 14,
                                fill: "red",
                            }
                            strong { "Unauthorised" }
                        }
                    }
                }
                td {
                    button {
                        class: "button is-warning is-small",
                        style: "height: 25px",
                        "current-state": "{value.database_state}",
                        onclick: move |_| {
                            let db_info = value.clone();
                            let client = connect.client.clone();
                            let current_state: &str = &value.database_state.to_lowercase();
                            let need_reload_setter = need_reload_setter.clone();
                            if current_state == "locked" {
                                // 这里做解锁操作，当一个库被锁定时，我们不能直接对它进行卸载
                                // 所以说，当库为锁定状态，我们先对它进行解锁。
                                cx.spawn(async move {
                                    let res = database::unlock_db(client, &db_info.name).await;
                                    if let Err(_e) = res {
                                        // println!("{}", e);
                                    }
                                    need_reload_setter(true);
                                });
                            } else {
                                // 这里是锁定操作
                                cx.spawn(async move {
                                    let res = database::lock_db(client, &db_info.name).await;
                                    if let Err(_e) = res {
                                        // println!("{}", e);
                                    }
                                    need_reload_setter(true);
                                });
                            }
                        },
                        "{unload_title}"
                    }
                }
            }
        }
    });

    if display_list.0.is_empty() || *need_reload.0 {
        let display_list_setter = display_list.1.clone();
        let need_reload_setter = need_reload.1.clone();
        cx.spawn(async move {
            let client = connect.client.clone();
            let usa_db_list = usable_db_list.clone();

            let mut list = db_list_info(client, usa_db_list).await;

            list.sort_by(|a, b| {
                let (a_name, b_name) = (a.name.clone(), b.name.clone());
                
                let mut a_weight = 0;
                let mut b_weight = 0;

                for c in a_name.chars() {
                    a_weight += c as u32;
                }

                for c in b_name.chars() {
                    b_weight += c as u32;
                }

                a_weight.cmp(&b_weight)
            });

            display_list_setter(list);
            need_reload_setter(false);
        });
    }

    cx.render(rsx! {
        div {
            class: "filed has-addons",
            style: "float: right",
            p {
                class: "control",
                button {
                    class: "button is-small",
                    onclick: move |_| {
                        let need_reload_setter = need_reload.1.clone();
                        need_reload_setter(true);
                    },
                    Icon {
                        icon: Shape::Refresh,
                    }
                }
            }
            br {}
        }
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
            tbody { display_rsx }
        }
    })
}
