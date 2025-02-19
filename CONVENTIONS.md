- Rust Leptos client side rendering web app
- To test the app, run `trunk serve --open` in the terminal
- Always run the above command to test the app on each change
- Fix any errors in the app from the terminal output (if any)

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
