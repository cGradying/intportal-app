# IntPortal

An unofficial student portal for PUP's SIS on iOS and Android. It shows your schedule, the next class, your grades with a real GPA, and class reminders, in a hybrid legacy and pixel-art look.

It is a from-scratch rebuild of the macOS IntPortal app. The old app's docs live in `docs/legacy/` and are the reference for every feature.

## Layout

- `crates/intportal-core` is the Rust core: SIS access, parsers, domain logic and SQLite.
- `ui/` is the Svelte 5 interface.
- `src-tauri/` is the Tauri 2 shell for iOS, Android, macOS and Windows.
- `docs/adr/` records the decisions that are hard to reverse.
- `CONTEXT.md` is the glossary.

## Status

Phase 0: foundation and spikes. The plan and parts are on the wayfinder map issue.

Only your own account is supported, which is what PUP's terms allow.
