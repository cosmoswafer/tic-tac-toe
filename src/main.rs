use leptos::*;
mod game;
use game::TicTacToe;

fn main() {
    mount_to_body(|| {
        view! { 
            <div id="game-root">
                <TicTacToe/>
            </div>
        }
    })
}
