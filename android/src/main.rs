use dioxus::prelude::*;
use dioxus_logger::tracing::Level;

mod ui;
mod state;

use ui::App;

fn main() {
    dioxus_logger::init(Level::DEBUG);
    dioxus::launch(App);
}
