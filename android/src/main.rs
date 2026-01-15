use dioxus::prelude::*;

mod ui;
mod state;

use ui::App;

fn main() {
    dioxus::launch(App);
}
