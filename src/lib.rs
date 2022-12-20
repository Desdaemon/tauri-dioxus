#![allow(non_snake_case)]

use dioxus::prelude::*;

#[cfg(target_os = "android")]
use wry::android_binding;

mod todo;

/// Entry point of the program.
#[no_mangle]
#[inline(never)]
pub extern "C" fn start_app() {
    #[cfg(target_os = "android")]
    android_binding!(com_example, tauri_1dioxus, _start_app);
    _start_app()
}

fn _start_app() {
    stop_unwind(main);
}

fn stop_unwind<F: FnOnce() -> T, T>(f: F) -> T {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(t) => t,
        Err(err) => {
            eprintln!("attempt to unwind out of `rust` with err: {:?}", err);
            std::process::abort()
        }
    }
}

// Dioxus-specific code

fn main() {
    #[cfg(target_family = "wasm")]
    dioxus_web::launch(todo::app);

    #[cfg(not(target_family = "wasm"))]
    dioxus_desktop::launch(todo::app);
}

#[cfg(feature = "inline-css")]
pub fn Styles(cx: Scope) -> Element {
    const STYLES: &str = include_str!(concat!(env!("OUT_DIR"), "/styles.css"));

    cx.render(rsx! {
        style {STYLES}
    })
}

#[cfg(not(feature = "inline-css"))]
pub fn Styles(_: Scope) -> Element {
    VNode::empty()
}
