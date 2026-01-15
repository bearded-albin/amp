use dioxus::prelude::*;
use amp_core::models::Address;
use crate::state::AppStateManager;

#[component]
pub fn AddressCard(address: Address) -> Element {
    let mut state = use_context::<Signal<AppStateManager>>();
    let id = address.id.clone();
    let id_delete = address.id.clone();
    
    rsx! {
        div {
            style: "border: 1px solid #ddd; padding: 10px; margin: 5px 0; border-radius: 5px;",
            h3 { "{address.name}" }
            p { "Street: {address.street}" }
            if let Some((lat, lon)) = address.get_coordinates() {
                p { "GPS: {lat}, {lon}" }
            }
            button {
                onclick: move |_| {
                    state.write().toggle_address(&id);
                },
                if address.active { "Deactivate" } else { "Activate" }
            }
            button {
                onclick: move |_| {
                    state.write().remove_address(&id_delete);
                },
                "Delete"
            }
        }
    }
}

#[component]
pub fn AddressForm() -> Element {
    let mut state = use_context::<Signal<AppStateManager>>();
    let mut name = use_signal(|| String::new());
    let mut street = use_signal(|| String::new());

    rsx! {
        form {
            onsubmit: move |_| {
                if !name().is_empty() && !street().is_empty() {
                    let mut addr = Address {
                        id: uuid_v4(),
                        name: name(),
                        street: street(),
                        coordinates: None,
                        active: true,
                    };
                    // Set default Malmö coordinates
                    addr.set_coordinates(55.6050, 13.0038);
                    state.write().add_address(addr);
                    name.set(String::new());
                    street.set(String::new());
                }
            },
            div {
                label { "Address Name: " }
                input {
                    r#type: "text",
                    value: name(),
                    oninput: move |e| name.set(e.value()),
                    placeholder: "e.g., Home",
                }
            }
            div {
                label { "Street: " }
                input {
                    r#type: "text",
                    value: street(),
                    oninput: move |e| street.set(e.value()),
                    placeholder: "e.g., Storgatan 1",
                }
            }
            button { r#type: "submit", "Add Address" }
        }
    }
}

fn uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    
    let nanos = duration.subsec_nanos();
    format!("addr-{}-{}", duration.as_secs(), nanos)
}
