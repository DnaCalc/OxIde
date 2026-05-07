# DNA OxIde Native Host Scaffold

This `src-tauri` tree is the W341 native scaffold for **DNA OxIde**.

Important boundaries:

- The Rust crate is dependency-free in W341 so `cargo test --manifest-path apps/dna-oxide/src-tauri/Cargo.toml` stays deterministic and offline.
- `tauri.conf.json` records product metadata for the future Tauri host.
- `src/commands.rs` lists command placeholders only; W344 owns real command adapters.
- `src/services.rs` keeps real execution, native runtime, COM runtime, fake Immediate responses, and fake debug data claims false.
- Real runtime/debug/Immediate/COM behavior requires OxVba-backed adapter tests before any claim can change.
