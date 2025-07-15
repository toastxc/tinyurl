//! Run with:
//!
//! ```sh
//! dx serve --platform web
//! ```

#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

type CommentType = String;
static CSS: Asset = asset!("assets/bulma/css/bulma.css");
fn error_page(error: impl Into<String> + std::fmt::Display) -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        p { class: "title is-1", "Error" }
        p { class: "subtitle is-3", "{error}" }
    }
}
#[component]
fn MyModal(url: Signal<String>, modal_open: Signal<bool>) -> Element {
    // let url_error = use_signal(|| None);
    // url.to_string();

    rsx! {
        div { class: "modal is-active",
            div { class: "modal-background",
                div { class: "modal-card",
                    form {

                        div { class: "field",
                            label { class: "label", "URL" }
                            input {
                                class: "input",
                                r#type: "text",
                                oninput: move |event| url.set(event.value()),
                                value: "https://",
                            }
                        }

                        div { class: "field",
                            label { class: "label", "Path (Premium Only)" }
                            input {
                                class: "input",
                                r#type: "text",
                                disabled: true,
                                placeholder: "12345",
                            }
                        }
                        a {
                            onclick: move |_| async move {
                                redir_create_rand(url.to_string()).await;
                                modal_open.set(false);
                            },
                            r#type: "link",
                            class: "button is-link",
                            "Create"
                        }
                        a {
                            onclick: move |_| async move { modal_open.set(false) },
                            class: "button is-link is-light",
                            "Cancel"
                        }
                    }
                }
            }
        }
    }
}

fn app() -> Element {

    // let global_error = use_signal(||None);


    let mut modal_open = use_signal(|| false);
    let mut url = use_signal(String::new);
    // load futures on startup
    let server_future = use_server_future(redirs_read)?;

    let Some(Ok(caddy_file)) = server_future.read().clone() else {
        return error_page("Could not connect to Dioxus backend");
    };

    let comments_rendered = caddy_file.into_iter().map(|(c1, c2)| {
        let mut title = use_signal(|| c1.clone());

        rsx! {
            tr {
                td { "https://t.toastxc.xyz{c1}" }
                td { "{c2}" }
                td { class:"is-danger",

                    a {
                        class: "button is-danger",
                        onclick: move |_| async move {
                            redir_delete(title.to_string()).await;
                        },
                        "Delete"
                    }
                }
            }
        }
    });

    rsx! {

        document::Stylesheet { href: CSS }

        br {}


        //         <div class:"columns is-mobile is-centered">
        //   <div class="column is-half">
        //     <p class="bd-notification is-primary">
        //       <code class="html">is-half</code><br />
        //     </p>
        //   </div>
        // </div>

        div { class: "columns is-mobile is-centered",
            div { class: "column is-half",


                table { class: "table is-bordered is-striped  is-hoverable is-fullwidth",
                    thead {
                        tr {

                            // <p> gkfkfk <p>
                            // <p> {"fjfjdjd"} <\p>
                            th { "Unique ID" }

                            th { "Destination" }
                            th { "Actions" }
                        }
                    }
                    {comments_rendered}
                }
               a {
                    class: "button is-primary  is-fullwidth",
                    onclick: move |_| async move { modal_open.set(true) },
                    "+"
                }
            
            }
        }





        if modal_open() {
            MyModal { url, modal_open }
        }
        // <button class="button ">Submit</button>



        // table { class: "table is-bordered is-striped is-narrow is-hoverable",
        //     thead {
        //         tr {
        //
        //             // <p> gkfkfk <p>
        //             // <p> {"fjfjdjd"} <\p>
        //             th { "Unique ID" }
        //
        //             th { "Destination" }
        //             th { "Actions" }
        //         }
        //     }
        //     {comments_rendered}
        // }
    }
}
// #[cfg(feature = "server")]
#[cfg(feature = "server")]
const DB_PATH: &'static str = "db.json";
#[cfg(feature = "server")]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Db {
    /// path | URL
    pub redirs: HashMap<String, String>,
}
#[cfg(feature = "server")]
impl Db {
    pub fn save(&self) {
        let mut caddy = CaddyFile::new();
        for x in &self.redirs {
            caddy.route_new(x.0, x.1);
        }

        std::fs::write(DB_PATH, serde_json::to_string_pretty(&self).unwrap()).unwrap();
        std::fs::write("caddy.json", caddy.export()).unwrap();
    }
}

#[cfg(feature = "server")]
use lazy_static::lazy_static;
#[cfg(feature = "server")]
use std::sync::Arc;
#[cfg(feature = "server")]
use tokio::sync::RwLock;

#[cfg(feature = "server")]
lazy_static! {
    /// This is an example for using doc comment attributes


    static ref DB: Arc<RwLock<Db>> = Arc::new(RwLock::new(serde_json::from_slice(&std::fs::read(DB_PATH).unwrap()).unwrap()));
}

// CRUD
#[server]
async fn redir_create(path: String, url: String) -> Result<(), ServerFnError> {
    if let Err(error) = CaddyFile::caddy_validate(&path, &url) {
        return Err(ServerFnError::Args(error));
    };
    DB.write().await.redirs.insert(path, url);
    DB.read().await.save();
    Ok(())
}

#[server]
async fn redir_create_rand(url: String) -> Result<(), ServerFnError> {
    use rand::Rng;
    redir_create(
        format!("/{}", rand::thread_rng().gen_range(10000..99999)),
        url,
    )
    .await
}

#[server]
async fn redirs_read() -> Result<HashMap<String, String>, ServerFnError> {
    Ok(DB.read().await.redirs.clone())
}
#[server]
async fn redir_read(path: String) -> Result<Option<String>, ServerFnError> {
    Ok(DB.read().await.redirs.get(&path).cloned())
}

#[server]
async fn redir_delete(path: String) -> Result<(), ServerFnError> {
    DB.write().await.redirs.remove(&path);
    DB.read().await.save();
    Ok(())
}

fn main() {
    dioxus::launch(app);
}

use crate::data::caddy::CaddyFile;

mod data;
