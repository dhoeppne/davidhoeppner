use dioxus::prelude::*;

use crate::pages::{Home, About, Experience, Work, Contact};

#[derive(Clone, Routable, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {},
        #[route("/about/:id")]
        About { id: i32 },
        #[route("/experience")]
        Experience {},
        #[route("/work")]
        Work {},
        #[route("/contact")]
        Contact {},
}

#[component]
pub fn Wrapper() -> Element {
    rsx! {
        Header {}
        Outlet::<Route> {}
        Footer {}
    }
}

#[component]
fn Header() -> Element {
    rsx! {
        nav {
            ul {
                li {
                    Link {
                        to: Route::Home {},
                        "Home"
                    }
                }
                li {
                    Link {
                        to: Route::About {id: 1},
                        "About"
                    }
                }
                li {
                    Link {
                        to: Route::Experience {},
                        "Experience"
                    }
                }
                li {
                    Link {
                        to: Route::Work {},
                        "Work"
                    }
                }
                li {
                    Link {
                        to: Route::Contact {},
                        "Contact"
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        footer {
            "Footer"
        }
    }
}