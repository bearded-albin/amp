use dioxus::prelude::*;
use crate::state::AppStateManager;

pub mod components;
pub mod pages;

pub use components::*;
pub use pages::*;

#[component]
pub fn App() -> Element {
    let state = use_signal(|| AppStateManager::new());
    let mut current_page = use_signal(|| "home");
    
    use_context_provider(|| state);

    rsx! {
        div {
            style: "font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; background: #ecf0f1; min-height: 100vh;",
            // Navigation
            nav {
                style: "background: #2c3e50; color: white; padding: 15px; display: flex; gap: 20px; box-shadow: 0 2px 4px rgba(0,0,0,0.1);",
                button {
                    onclick: move |_| current_page.set("home"),
                    style: if current_page() == "home" { 
                        "background: #3498db; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    } else { 
                        "background: transparent; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    },
                    "Home"
                }
                button {
                    onclick: move |_| current_page.set("add"),
                    style: if current_page() == "add" { 
                        "background: #3498db; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    } else { 
                        "background: transparent; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    },
                    "Add"
                }
                button {
                    onclick: move |_| current_page.set("addresses"),
                    style: if current_page() == "addresses" { 
                        "background: #3498db; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    } else { 
                        "background: transparent; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    },
                    "Addresses"
                }
                button {
                    onclick: move |_| current_page.set("settings"),
                    style: if current_page() == "settings" { 
                        "background: #3498db; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    } else { 
                        "background: transparent; color: white; border: none; padding: 8px 15px; border-radius: 4px; cursor: pointer;" 
                    },
                    "Settings"
                }
            }

            // Page content
            match current_page() {
                "home" => rsx! { Home {} },
                "add" => rsx! { AddAddress {} },
                "addresses" => rsx! { Addresses {} },
                "settings" => rsx! { Settings {} },
                _ => rsx! { Home {} },
            }
        }
    }
}
