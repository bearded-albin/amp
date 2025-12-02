use iced::widget::{button, column, combo_box, container, row, scrollable, text_input};
use iced::Element;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Adress {
    gata: String,
    gatunummer: String,
    postnummer: String,
    aktiv: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lagring {
    adress(Adress),
}

impl std::fmt::Display for Lagring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lagring::adress(adress) => "test",
            }
        )
    }
}

#[derive(Debug, Clone)]
enum Message {
    GataChanged(String),
    GatunummerChanged(String),
    PostnummerChanged(String),
    AddAddressButtonPressed,
    Selected(Adress),
    OptionHovered(Adress),
    Closed,
}

pub fn main() -> iced::Result {
    iced::run("amp", Adress::update, Adress::view)
}

impl Adress {
    fn new() -> Self {
        Self {
            gata: "".to_string(),
            gatunummer: "".to_string(),
            postnummer: "".to_string(),
            aktiv: false,
        }
    }
    fn view(state: &Adress) -> Element<'_, Message> {
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
                combo_box(
                    &state.gata,
                    "Select your favorite fruit...",
                    Option::from(state.gata.as_ref()),
                    Message::Selected(*state),)
            ]))
            .padding(10)
            .style(container::rounded_box)
        ]
            .into()
    }

    fn update(state: &mut Adress, message: Message) {
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
            Message::Selected(adress) => {
                state.gata = adress.gata;
            }
            Message::OptionHovered(adress) => {
                state.gata = adress.gata.to_string();
            }
            Message::Closed => {
                state.gata = state.gata.clone();
            }
        }
    }
}

impl Default for Adress {
    fn default() -> Self {
        Adress::new()
    }
}
