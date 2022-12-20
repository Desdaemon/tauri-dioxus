fn main() {
    #[cfg(target_family = "wasm")]
    {
        tauri_dioxus::start_app();
    }
}
