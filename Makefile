build-debug:
	rustup run nightly cargo build 
	cp ./target/debug/toast ./toast-node-wrapper/
build-production:
	rustup run nightly cargo build --release
	cp ./target/release/toast ./toast-node-wrapper/
copy-linux-toast:
	docker build -t temprust .
	container := $(docker create temprust)
	docker create ${container}
	docker cp ${container}:/opt/app/target .
