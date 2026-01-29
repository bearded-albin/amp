use crate::ui::StoredAddress;
use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct AddressInputState {
    pub gata: String,
    pub gatunummer: String,
    pub postnummer: String,
}

#[component]
pub fn Adresser(
    stored_addresses: Vec<StoredAddress>,
    on_toggle_active: EventHandler<usize>,
    on_remove_address: EventHandler<usize>,
) -> Element {
    let mut input = use_signal(AddressInputState::default);

    rsx! {
        div { class: "stored-addresses",
            h2 { "Adresser" }

            div { class: "input-section",
                div { class: "input-group",
                    input {
                        class: "address-input",
                        placeholder: "Gata",
                        value: "{input.read().gata}",
                        onchange: move |evt: Event<FormData>| {
                            input.write().gata = evt.value();
                        },
                    }
                    input {
                        class: "address-input",
                        placeholder: "Gatunummer",
                        value: "{input.read().gatunummer}",
                        onchange: move |evt: Event<FormData>| {
                            input.write().gatunummer = evt.value();
                        },
                    }
                    input {
                        class: "address-input",
                        placeholder: "Postnummer",
                        value: "{input.read().postnummer}",
                        onchange: move |evt: Event<FormData>| {
                            input.write().postnummer = evt.value();
                        },
                    }
                }
            }

            div { id: "addressList",
                {stored_addresses.iter().enumerate().map(|(idx, addr)| {
                    let validation_indicator = if addr.valid {
                        rsx! { span { class: "valid-indicator", "✓" } }
                    } else {
                        rsx! { span { class: "invalid-indicator", "✗" } }
                    };

                    let active_class = if addr.active { "active" } else { "inactive" };

                    rsx! {
                        div { key: "{idx}", class: "address-item {active_class}",
                            div { class: "address-header",
                                {validation_indicator}
                                div { class: "address-text",
                                    "{addr.gata} {addr.gatunummer}, {addr.postnummer}"
                                }
                            }
                            div { class: "address-controls",
                                button {
                                    class: "toggle-button",
                                    onclick: move |_| on_toggle_active.call(idx),
                                    if addr.active { "Dölj" } else { "Visa" }
                                }
                                button {
                                    class: "remove-button",
                                    onclick: move |_| on_remove_address.call(idx),
                                    "×"
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}
