use dioxus::prelude::*;
use dioxus_web::launch;

fn main() {
    // Initialisation du logger pour le développement
    launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        div {
            h1 { "Portfolio - Mathieu Piton" }
            p { "En construction..." }
        }
    }
}
