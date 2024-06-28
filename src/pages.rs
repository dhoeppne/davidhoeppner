#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info};

#[component]
pub fn About() -> Element {
    rsx! {
      div {
        "About"
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
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

#[component]
pub fn Home() -> Element {
    let mut count = use_signal(|| 0);
    let mut text = use_signal(|| String::from("..."));

    rsx! {

    div {
      h1 { "High-Five counter: {count}" }
      button { onclick: move |_| count += 1, "Go up!" }
      button { onclick: move |_| count -= 1, "Go down!" }
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
      p { "Server data: {text}" }
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