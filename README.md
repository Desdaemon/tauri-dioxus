# tauri-dioxus

[Dioxus] integration with [Tauri]'s `wry` template. Preconfigured with [Tailwind CSS] and [DaisyUI].

Runs on mobile[^1], desktop and web.

## Configuration

Tools needed:
- [tauri-mobile]
- An NPM package manager and `npx`
- [trunk], [wasm-bindgen-cli] and [wasm-opt] (for web)
- Necessary mobile frameworks (Xcode, Android Studio)

```bash
npm install # or yarn, pnpm install, etc.
cargo mobile init
cargo run          # desktop
cargo android run  # Android
cargo apple run    # Apple
trunk serve        # web
```

### Android
Original:

> From my setup, I also need to add `abiFilters += listOf("arm64-v8a")` under `create("arm")` branch in `:app`'s '`build.gradle.kts`.
> 
> This is probably different from users env, so I didn't add to the script.

### iOS
Original:

> Must run Xcode on rosetta. Goto Application > Right Click Xcode > Get Info > Open in Rosetta.
>
> If you are using M1, you will have to run `cargo build --target x86_64-apple-ios` instead of `cargo apple build` if you want to run in simulator.
>
> Otherwise, it's all `cargo apple run` when running in actual device.

[^1]: Only tested on Android.

[Dioxus]: https://dioxuslabs.com
[Tauri]: https://tauri.app
[Tailwind CSS]: https://tailwindcss.com
[DaisyUI]: https://daisyui.com
[tauri-mobile]: https://github.com/tauri-apps/tauri-mobile
[trunk]: https://trunkrs.dev
[wasm-bindgen-cli]: https://crates.io/crates/wasm-bindgen-cli
[wasm-opt]: https://github.com/webassembly/binaryen