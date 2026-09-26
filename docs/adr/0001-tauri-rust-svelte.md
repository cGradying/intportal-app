# Tauri 2 with a Rust core and a Svelte 5 UI

The first IntPortal was a macOS SwiftUI app. Moving it one slice at a time was too slow, and phones (iOS and Android) are now the main platforms. So we are rebuilding from scratch on Tauri 2.

- The Rust core, `intportal-core`, owns SIS access, parsing, domain logic and SQLite.
- One Svelte 5 web UI serves every platform.

## Considered options

- **Flutter with a Rust core.** Flutter's own renderer suits pixel art. We rejected it because the landing and island prototypes and the notes editor are already HTML and JS, and would have to be rewritten in Dart.
- **A Rust core with native SwiftUI and Compose UIs.** The most native feel, but two UI codebases for a solo builder.
- **React Native.** The bridge to Rust is fiddlier, and there is no gain over a webview for a custom-drawn look.

## Consequences

- Anything a webview can't do becomes a small native add-on in Swift or Kotlin: home-screen widgets, Live Activities, and possibly hidden-webview SIS sign-in (see spike S0).
- The old Swift code is a reference only. Logic is ported to Rust, and its test fixtures become the literal expected values.
