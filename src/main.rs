use leptos::*;
mod game;
use game::TicTacToe;

fn main() {
    mount_to_body(|| {
        view! { 
            <div id="game-root">
                <Style id="game-styles">{include_str!("../public/styles.css")}</Style>
                <TicTacToe/>
            </div>
        }
    })
}
