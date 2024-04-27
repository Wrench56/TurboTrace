REM Call this from the root of the project folder

cd client\src-tauri
cargo fmt -- --check
cargo clippy -- -D warnings
cargo check
cd ..\..