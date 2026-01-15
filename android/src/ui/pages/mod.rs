use dioxus::prelude::*;
use crate::state::AppStateManager;
use crate::ui::components::{AddressCard, AddressForm};

#[component]
pub fn Home() -> Element {
    let state = use_context::<Signal<AppStateManager>>();
    let count = state.read().address_count();

    rsx! {
        div {
            style: "padding: 20px;",
            h1 { "AMP - Malmö Parking Alerts" }
            p { "You have {count} saved addresses." }
            if count == 0 {
                p { "Click 'Add' to add your first address!" }
            } else {
                p { "Active addresses are monitored for cleaning schedules." }
            }
        }
    }
}

#[component]
pub fn AddAddress() -> Element {
    rsx! {
        div {
            style: "padding: 20px;",
            h1 { "Add Address" }
            AddressForm {}
        }
    }
}

#[component]
pub fn Addresses() -> Element {
    let state = use_context::<Signal<AppStateManager>>();
    let addresses = state.read().get_addresses();

    rsx! {
        div {
            style: "padding: 20px;",
            h1 { "Your Addresses" }
            if addresses.is_empty() {
                p { "No addresses yet." }
            } else {
                div {
                    for address in addresses {
                        AddressCard {
                            key: "{address.id}",
                            address,
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn Settings() -> Element {
    let mut state = use_context::<Signal<AppStateManager>>();

    rsx! {
        div {
            style: "padding: 20px;",
            h1 { "Settings" }
            button {
                onclick: move |_| {
                    state.write().clear_all();
                },
                style: "background: #e74c3c; color: white; padding: 10px; border: none; border-radius: 5px; cursor: pointer;",
                "Clear All Addresses"
            }
        }
    }
}
