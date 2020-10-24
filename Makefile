build-debug:
	rustup run nightly cargo build 
	cp ./target/debug/toast ./toast-node-wrapper/
build-debug-install: build-debug install-locally
build-production:
	rustup run nightly cargo build --release
	cp ./target/release/toast ./toast-node-wrapper/
build-production-install: build-production install-locally

copy-linux-toast:
	docker build -t temprust .
	container := $(docker create temprust)
	docker create ${container}
	docker cp ${container}:/opt/app/target .


installDir = $(shell /usr/bin/env node ./toast-node-wrapper/binary-management/printInstallDirectory.js);
install-locally: 
	#if the installDir doesn't exist, make it
	[ -d '$(installDir)' ] || mkdir -p $(installDir)
	node ./toast-node-wrapper/binary-management/printBinaryPath.js
	cp ./toast-node-wrapper/toast $(shell /usr/bin/env node ./toast-node-wrapper/binary-management/printBinaryPath.js)