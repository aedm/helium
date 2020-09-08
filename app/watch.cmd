set RUSTFLAGS=-A dead_code -A unused_variables -A unused_imports -A unused_mut
cargo watch --clear -x "check --color=always"