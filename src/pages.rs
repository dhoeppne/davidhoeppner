#![allow(non_snake_case)]

use dioxus::prelude::*;
use tracing::{info};


#[component]
fn ExternalLink(linkTo: String, text: String) -> Element {
  rsx! {
    Link {
      new_tab: true,
      to: linkTo,
      "{text}",
    }
  }
}

#[component]
pub fn About() -> Element {
  let skills = vec![
    "JavaScript (ES6+)",
    "TypeScript",
    "React",
    "Kubernetes",
    "Node.js",
    "Docker",
    "Python",
    "Jenkins",
  ];

  rsx! {
    div {
      p {
        "Hello! My name is David and I enjoy creating things that create other things. I had a meager start to web development back in 2013 when I experimented with editing some Tumblr themes, but I really started to get involved in 2018 when I interned for Intuit in Edmonton. Working on the QuickBooks Online devops team gave me a kickstart into tooling and operations."
      }
      p {
        "Fast-forward to today, and I've had the privilege of working at "
        ExternalLink { linkTo:"https://www.intuit.com/", text:"Intuit" }
        " for 5 years, both as an intern and full time engineer. I currently work on the AppFabric Team, which provides cross-app support for 100s of Intuit applications, including QuickBooks Online, TurboTax Online,and Mint."
      }
      p {
        "Here are a few technologies I've been working with recently:"
      }
      ul {
        for skill in skills {
          li { "{skill}" }
        }
      }
    }
  }
}

#[component]
pub fn Projects() -> Element {
    rsx! {
      div {
        h2 { "Some Things I’ve Built" }
        p { "put some projects here in for loop or something" }
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
  let email = "email@davidhoeppner.ca";
  rsx! {
    div {
      h2 { "Get in Touch" }
      p {
        "I currently work at Workday as a Software Engineer in Vancouver, Canada as a part of the Search team. Don't hesitate to reach out to me if you have any questions or just want to say hi!"
      }
      ExternalLink{linkTo: format!("mailto:{email}"), text: "Say Hello"}
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
    let testing = false;
    let first = rsx! { h1 { "Hi, my name is"}};
    let second = rsx! { h2 { "David Hoeppner"}};
    let third = rsx! { h3 { "I build things for people that build things for the web."}};
    let fourth = rsx! {
      p {
        "I’m a Canadian full-stack software developer specializing in Javascript-focused full-stack engineering. Currently, I’m focused on building accessible, human-centered software at "
      }
      ExternalLink { linkTo: "https://www.workday.com/", text: "Workday" }
      p { "." }
    };

    rsx! {
      if testing {
        div {
          h1 { "High-Five counter: {count}" }
          button { onclick: move |_| count += 1, "Go up!" }
          button { onclick: move |_| count -= 1, "Go down!" }
        }
      }
      else {
        div {
          { first }
          { second }
          { third }
          { fourth }
        }
      }


    }
}