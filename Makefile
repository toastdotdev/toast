build-debug:
	rustup run nightly cargo build
	cp ./target/debug/toast ./toast-node-wrapper/
build-production:
	rustup run nightly cargo build --release
	cp ./target/release/toast ./toast-node-wrapper/