use dioxus::prelude::*;
use dioxus_chessboard::{Chessboard, PieceSet, PlayerColor};

const _TAILWIND_URL: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut color = use_signal(|| PlayerColor::White);
    let mut pieces_set = use_signal(|| PieceSet::Standard);

    let _castling =
        "r3kbnr/ppp1qppp/2np4/4p3/2B1P1b1/P1N2N2/1PPP1PPP/R1BQK2R w KQkq - 1 6".to_string();

    let promotion = "rnbqkb1r/ppppn1P1/7p/8/8/4BN2/PPp1BPPP/RN1QK2R w KQkq - 2 9".to_string();

    rsx! {
        div {
            div {
                class: "w-1/3 border border-black",
                Chessboard {
                    color: color.read().to_owned(),
                    position: promotion,
                    pieces_set:  *pieces_set.read()
                }
            }
            button {
                onclick: move|_| color.write().flip(),
                class: "px-4 py-2 bg-blue-500 text-white font-medium rounded-md hover:bg-blue-600 focus:outline-none focus:ring-2 focus:ring-blue-300 focus:ring-opacity-50 transition duration-150 ease-in-out",
                "Flip the board"
            }
            input {
                r#type: "range",
                min: "1",
                max: "2",
                oninput: move |ev| {
                    // Update the state with the new value
                    let value = ev.value().parse::<u8>().expect("Slider value must be well defined");
                    match value {
                        1 =>  *pieces_set.write() = PieceSet::Standard,
                        2 =>  *pieces_set.write() = PieceSet::Funny,
                        _ => {}
                    }

                }
            }
        }
    }
}
