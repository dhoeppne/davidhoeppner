#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info};

#[component]
pub fn About(id: i32) -> Element {
    rsx! {
      div {
        "About {id}"
      }
    }
}

#[component]
pub fn Experience() -> Element {
    rsx! {
      div {
        "Experience"
      }
    }
}

#[component]
pub fn Work() -> Element {
    rsx! {
      div {
        "Work"
      }
    }
}

#[component]
pub fn Contact() -> Element {
    rsx! {
      div {
        "Contact"
      }
    }
}

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button {
                onclick: move |_| async move {
                    if let Ok(data) = get_server_data().await {
                        tracing::info!("Client received: {}", data);
                        text.set(data.clone());
                        post_server_data(data).await.unwrap();
                    }
                },
                "Get Server Data"
            }
            p { "Server data: {text}"}
        }
    }
}

#[server(PostServerData)]
async fn post_server_data(data: String) -> Result<(), ServerFnError> {
    info!("Server received: {}", data);
    Ok(())
}

#[server(GetServerData)]
async fn get_server_data() -> Result<String, ServerFnError> {
    Ok("Hello from the server!".to_string())
}