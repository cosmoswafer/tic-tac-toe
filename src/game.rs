use leptos::*;

#[component]
pub fn TicTacToe() -> impl IntoView {
    let (board, set_board) = create_signal(vec![String::new(); 9]);
    let (current_player, set_current_player) = create_signal(String::from("X"));
    let (winner, set_winner) = create_signal(String::new());

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
        if board.get()[index].is_empty() && winner.get().is_empty() {
            let mut new_board = board.get();
            new_board[index] = current_player.get();
            set_board(new_board.clone());
            
            if let Some(winner_player) = check_winner(new_board) {
                set_winner(winner_player);
            } else {
                set_current_player(if current_player.get() == "X" {
                    String::from("O")
                } else {
                    String::from("X")
                });
            }
        }
    };

    let reset_game = move |_| {
        set_board(vec![String::new(); 9]);
        set_current_player(String::from("X"));
        set_winner(String::new());
    };

    view! {
        <div class="game-container">
            <h1>"Tic Tac Toe"</h1>
            <div class="status">
                {move || {
                    if !winner.get().is_empty() {
                        format!("Winner: {}", winner.get())
                    } else {
                        format!("Current player: {}", current_player.get())
                    }
                }}
            </div>
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
