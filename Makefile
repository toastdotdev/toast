build-debug:
	rustup run nightly cargo build
	cp ./target/debug/toast ./toast-node-wrapper/