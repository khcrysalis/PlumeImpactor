.PHONY: macos clean


macos:
# 	env /usr/bin/arch -x86_64 cargo build --bin plumeimpactor --release --target x86_64-apple-darwin
	cargo build --bin plumeimpactor --release --target aarch64-apple-darwin

	mkdir -p build/macos
	cp -R package/macos/PlumeImpactor.app build/macos/PlumeImpactor.app
# 	lipo -create -output build/macos/PlumeImpactor.app/Contents/MacOS/plumeimpactor \
# 		target/x86_64-apple-darwin/release/plumeimpactor \
# 		target/aarch64-apple-darwin/release/plumeimpactor
	cp target/aarch64-apple-darwin/release/plumeimpactor build/macos/PlumeImpactor.app/Contents/MacOS/plumeimpactor

clean:
	rm -rf build/
