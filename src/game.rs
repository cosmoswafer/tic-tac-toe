use leptos::*;
use std::time::Duration;

#[component]
pub fn TicTacToe() -> impl IntoView {
    let (board, set_board) = create_signal(vec![String::new(); 9]);
    let (current_player, set_current_player) = create_signal(String::from("❌"));
    let (winner, set_winner) = create_signal(String::new());
    let (x_time_left, set_x_time_left) = create_signal(10);
    let (o_time_left, set_o_time_left) = create_signal(10);
    let (game_started, set_game_started) = create_signal(false);

    let check_winner = move |board: Vec<String>| {
        let winning_combinations = [
            [0, 1, 2], [3, 4, 5], [6, 7, 8], // Rows
            [0, 3, 6], [1, 4, 7], [2, 5, 8], // Columns
            [0, 4, 8], [2, 4, 6],            // Diagonals
        ];

        for combo in winning_combinations.iter() {
            if !board[combo[0]].is_empty()
                && board[combo[0]] == board[combo[1]]
                && board[combo[1]] == board[combo[2]]
            {
                return Some(board[combo[0]].clone());
            }
        }
        None
    };

    let handle_click = move |index: usize| {
        if board.get()[index].is_empty() && winner.get().is_empty() && game_started.get() {
            let mut new_board = board.get();
            new_board[index] = current_player.get();
            set_board.set(new_board.clone());
            
            if let Some(winner_player) = check_winner(new_board) {
                set_winner.set(winner_player);
            } else {
                // Reset timer for next player
                if current_player.get() == "❌" {
                    set_x_time_left.set(10);
                    set_current_player.set(String::from("⭕"));
                } else {
                    set_o_time_left.set(10);
                    set_current_player.set(String::from("❌"));
                }
            }
        }
    };

    let reset_game = move |_| {
        set_board.set(vec![String::new(); 9]);
        set_current_player.set(String::from("❌"));
        set_winner.set(String::new());
        set_x_time_left.set(10);
        set_o_time_left.set(10);
        set_game_started.set(false);
    };

    let start_game = move |_| {
        set_game_started.set(true);
    };

    // Timer logic
    let timer = set_interval_with_handle(
        move || {
            if winner.get().is_empty() && game_started.get() {
                if current_player.get() == "❌" {
                    let time = x_time_left.get();
                    if time > 0 {
                        set_x_time_left.set(time - 1);
                    } else {
                        set_winner.set(String::from("⭕"));
                    }
                } else {
                    let time = o_time_left.get();
                    if time > 0 {
                        set_o_time_left.set(time - 1);
                    } else {
                        set_winner.set(String::from("❌"));
                    }
                }
            }
        },
        Duration::from_secs(1),
    );

    on_cleanup(move || {
        if let Ok(handle) = timer {
            handle.clear();
        }
    });

    view! {
        <div class="game-container">
            <h1>"Tic Tac Toe"</h1>
            <div class="status">
                {move || {
                    if !game_started.get() {
                        String::from("Click Start to begin!")
                    } else if winner.get().is_empty() {
                        format!("Current player: {}", current_player.get())
                    } else {
                        String::new()
                    }
                }}
            </div>
            {move || {
                if !game_started.get() && winner.get().is_empty() {
                    view! {
                        <button class="start-button" on:click=start_game>"Start Game"</button>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
            <div class="timers">
                <div class="timer">
                    {"❌ Time: "}{move || x_time_left.get()}{" seconds"}
                </div>
                <div class="timer">
                    {"⭕ Time: "}{move || o_time_left.get()}{" seconds"}
                </div>
            </div>
            {move || {
                if !winner.get().is_empty() {
                    view! {
                        <div class="modal-overlay">
                            <div class="modal">
                                <h2>"Congratulations!"</h2>
                                <p>{"Winner: "}{winner.get()}</p>
                                <button on:click=reset_game>"Play Again"</button>
                            </div>
                        </div>
                    }
                } else {
                    view! { <div></div> }
                }
            }}
            <div class="board">
                {(0..9)
                    .map(|i| {
                        view! {
                            <button
                                class="cell"
                                on:click=move |_| handle_click(i)
                            >
                                {move || board.get()[i].clone()}
                            </button>
                        }
                    })
                    .collect::<Vec<_>>()}
            </div>
            <button class="reset-button" on:click=reset_game>"Reset Game"</button>
        </div>
    }
}
