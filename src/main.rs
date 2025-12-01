use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::Element;

#[derive(Default)]
struct State {
    gata: String,
    gatunummer: String,
    postnummer: String,
}

#[derive(Debug, Clone)]
enum Message {
    GataChanged(String),
    GatunummerChanged(String),
    PostnummerChanged(String),
    AddAddressButtonPressed,
}

pub fn main() -> iced::Result {
    iced::run("amp", update, view)
}

fn view(state: &State) -> Element<'_, Message> {
    column![
        container(row![
            text_input("Gata", &state.gata).on_input(Message::GataChanged),
            text_input("Gatunummer", &state.gatunummer).on_input(Message::GatunummerChanged),
            text_input("Postnummer", &state.postnummer).on_input(Message::PostnummerChanged),
            button("+").on_press(Message::AddAddressButtonPressed),
        ])
        .padding(10)
        .style(container::rounded_box),
        container(scrollable(column![
            //Parse stored addresses from JSON
            text(&state.gata)
        ]))
        .padding(10)
        .style(container::rounded_box)
    ]
    .into()
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::GataChanged(gata) => {
            state.gata = gata;
        }
        Message::GatunummerChanged(gatunummer) => {
            state.gatunummer = gatunummer;
        }
        Message::PostnummerChanged(postnummer) => {
            state.postnummer = postnummer;
        }
        Message::AddAddressButtonPressed => {
            //let _content = state.content.clone(); //Add to JSON list and write
        }
    }
}
