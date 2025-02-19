- Rust Leptos client side rendering web app
- To test the app, run `trunk serve --open` in the terminal

System Architecture:
1. Frontend (Client-Side Rendering)
   - Uses Leptos framework for reactive web UI
   - Components render in WebAssembly (WASM)
   - Handles user interactions and state management
   
2. Build/Development
   - Trunk for building and serving the application
   - Compiles Rust to WebAssembly
   - Bundles static assets (HTML, CSS)

3. Runtime
   - Runs entirely in the browser
   - No backend server required (static hosting)
   - State managed client-side through Leptos signals


# Architecture

This project follows a modern web application architecture using Rust and Leptos:

1. Project Structure
   - /src: Core Rust source code
     - main.rs: Application entry point
     - game.rs: Game logic and components
   - /public: Static assets
   - /target: Build outputs (compiled WASM)
   
2. Component Architecture
   - Modular components using Leptos
   - Reactive state management
   - Event-driven user interactions

3. Build Pipeline
   - Trunk handles asset bundling
   - Rust compiles to WASM
   - Static files served to browser

4. Runtime Architecture
   - Client-side only (no server)
   - WASM execution in browser
   - Reactive updates via Leptos


# API reference

## Leptos main functions mount_to_body

The `mount_to_body` function is a core Leptos utility that:
- Mounts a Leptos component as a child of the document body
- Takes a closure that returns the root component view
- Initializes the Leptos runtime and reactive system
- Handles WASM rendering of components to the DOM
- Is typically used as the main entry point for Leptos apps

```rust
use leptos::*;

fn main() {
    mount_to_body(|| {
        view! { 
            <div>
                <h1>"Hello from Leptos!"</h1>
                <MyComponent/>
            </div>
        }
    })
}

#[component]
fn MyComponent() -> impl IntoView {
    view! {
        <p>"This is a mounted component"</p>
    }
}
```

## TicTacToe Component example

The `TicTacToe` component implements a complete Tic-Tac-Toe game with the following features:

- Game board with 9 cells (3x3 grid)
- Two players: ❌ and ⭕
- Turn-based gameplay with timer (10 seconds per turn)
- Win detection for rows, columns, and diagonals
- Game state management:
  - Board state tracking
  - Current player tracking
  - Winner detection
  - Timer countdown
  - Game start/reset functionality
- UI Elements:
  - Game board grid
  - Player timers
  - Status messages
  - Start game button
  - Winner modal with replay option

Key Functions:
- `check_winner`: Checks for winning combinations
- `handle_click`: Processes player moves
- `reset_game`: Resets the game state
- `start_game`: Initiates a new game
- Timer interval: Manages countdown for each player's turn

# Give an short example of the above component with using signals to manage the state for demonstration #AI!
```rust

```
