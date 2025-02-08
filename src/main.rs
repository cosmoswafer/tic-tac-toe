use leptos::*;
mod game;
use game::TicTacToe;

fn main() {
    mount_to_body(|| view! { <TicTacToe/> })
}
