use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use dioxus_primitives::switch::SwitchThumb;
use dioxus_primitives::switch::Switch;

pub mod components;

#[derive(Clone, PartialEq)]
struct AddressItem {
    id: usize,
    address: String,
    postal: String,
    stored: bool,
}

static CSS: Asset = asset!("../ui/assets/style.css");
static COMP: Asset = asset!("../ui/assets/dx-components-theme.css");

#[component]
pub fn App() -> Element {
    let mut address_input = use_signal(|| String::new());
    let mut postal_input = use_signal(|| String::new());
    let mut items = use_signal(|| vec![]);
    let mut next_id = use_signal(|| 1usize);
    let mut show_dialog = use_signal(|| false);
    let mut delete_id = use_signal(|| None::<usize>);

    let add_address = move |_| {
        if !address_input().is_empty() && !postal_input().is_empty() {
            let new_item = AddressItem {
                id: *next_id.read(),
                address: address_input().clone(),
                postal: postal_input().clone(),
                stored: false,
            };
            items.write().push(new_item);
            address_input.set(String::new());
            postal_input.set(String::new());
            let current_id = *next_id.read();   
            next_id.set(current_id + 1); 
        }
    };

    let mut toggle_stored = move |id: usize| {
        if let Some(item) = items.write().iter_mut().find(|i| i.id == id) {
            item.stored = !item.stored;
        }
    };

    let mut open_delete_dialog = move |id: usize| {
        delete_id.set(Some(id));
        show_dialog.set(true);
    };

    let confirm_delete = move |_| {
        if let Some(id) = *delete_id.read() {
            items.write().retain(|i| i.id != id);
        }
        show_dialog.set(false);
        delete_id.set(None);
    };

    let cancel_delete = move |_| {
        show_dialog.set(false);
        delete_id.set(None);
    };

    rsx! {

        Stylesheet { href: CSS }
        Stylesheet { href: COMP }

        div {
            class: "app-container",

            div {
                class: "scroll-view",

                div {
                    class: "container",

                    // Header
                    div {
                        class: "header",
                        h1 {
                            class: "header-title",
                            "Adresshanterare"
                        }
                    }

                    // Form Section
                    div {
                        class: "form-section",

                        h2 {
                            class: "section-title",
                            "Lägg till adress"
                        }

                        div {
                            class: "form-group",
                            label { class: "form-label", "Adress" }
                            input {
                                class: "text-input",
                                value: "{address_input}",
                                placeholder: "T.ex. Gatan 123",
                                onchange: move |evt| {
                                    address_input.set(evt.value());
                                },
                            }
                        }

                        div {
                            class: "form-group",
                            label { class: "form-label", "Postnummer" }
                            input {
                                class: "text-input",
                                value: "{postal_input}",
                                placeholder: "T.ex. 12345",
                                onchange: move |evt| {
                                    postal_input.set(evt.value());
                                },
                            }
                        }

                        button {
                            class: "btn btn-primary",
                            onclick: add_address,
                            "Lägg till"
                        }
                    }

                    // Saved Addresses Section
                    div {
                        class: "addresses-section",

                        h2 {
                            class: "section-title",
                            "Sparade adresser"
                        }

                        if items().is_empty() {
                            div {
                                class: "empty-state",
                                p { "Ingen adresser lagrad ännu." }
                                p { "Lägg till din första adress ovan!" }
                            }
                        } else {
                            div {
                                class: "address-list",
                                for item in items() {
                                    AddressItemComponent {
                                        key: "{item.id}",
                                        item: item.clone(),
                                        on_toggle: move |_| toggle_stored(item.id),
                                        on_delete: move |_| open_delete_dialog(item.id),
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Delete Dialog
            if *show_dialog.read() {
                div {
                    class: "dialog-overlay",
                    div {
                        class: "dialog",
                        h3 { class: "dialog-title", "Bekräfta borttagning" }
                        p { class: "dialog-message", "Är du säker på att du vill ta bort denna adress?" }
                        div {
                            class: "dialog-buttons",
                            button {
                                class: "btn btn-danger",
                                onclick: confirm_delete,
                                "Bekräfta"
                            }
                            button {
                                class: "btn btn-secondary",
                                onclick: cancel_delete,
                                "Avbryt"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AddressItemComponent(
    item: AddressItem,
    on_toggle: EventHandler<()>,
    on_delete: EventHandler<()>,
) -> Element {
    let mut is_stored = use_signal(|| item.stored);

    rsx! {
        div {
            class: "address-card",

            div {
                class: "address-content",

                div {
                    class: "address-text",
                    p { class: "address-value", "{item.address}" }
                    p { class: "postal-value", "{item.postal}" }
                }

                div {
                    class: "address-controls",

                    button {
                        class: if *is_stored.read() { 
                            "btn btn-toggle btn-toggle-active" 
                        } else { 
                            "btn btn-toggle btn-toggle-inactive" 
                        },
                        onclick: move |_| {
                            is_stored.toggle();
                            on_toggle.call(());
                        },
                        if *is_stored.read() { "✓" } else { "○" }
                    }

                    button {
                        class: "btn btn-remove",
                        onclick: move |_| on_delete.call(()),
                        "✕"
                    }
                }
            }
        }
    }
}