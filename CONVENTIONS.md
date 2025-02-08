- Rust Leptos client side rendering web app
- To test the app, run `trunk serve --open` in the terminal
- Always run the above command to test the app on each change

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

