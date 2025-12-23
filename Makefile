
clean:
	@rm -rf ./dist
	@rm -rf ./build
# TODO: fix wxwidgets from not cross-compiling
# TODO: fix embedding manifest in windows build
macos:
	@make clean
# 	@cargo build --workspace --release --target x86_64-apple-darwin
	@cargo build --workspace --release --target aarch64-apple-darwin

	@mkdir -p ./dist/macos

# 	@lipo -create -output ./dist/macos/plumeimpactor ./target/x86_64-apple-darwin/release/plumeimpactor ./target/aarch64-apple-darwin/release/plumeimpactor
# 	@lipo -create -output ./dist/macos/plumeimpactor ./target/aarch64-apple-darwin/release/plumesign ./target/aarch64-apple-darwin/release/plumesign
	@strip ./target/aarch64-apple-darwin/release/plumeimpactor
	@strip ./target/aarch64-apple-darwin/release/plumesign
	@cp ./target/aarch64-apple-darwin/release/plumeimpactor ./dist/macos/plumeimpactor
	@cp ./target/aarch64-apple-darwin/release/plumesign ./dist/plumesign-macos-universal

	@cp -R package/macos/Impactor.app ./dist/macos/Impactor.app
	@mkdir -p ./dist/macos/Impactor.app/Contents/MacOS
	@VERSION=$$(awk '/\[workspace.package\]/,/^$$/' Cargo.toml | sed -nE 's/version *= *"([^"]*)".*/\1/p'); \
		/usr/libexec/PlistBuddy -c "Set :CFBundleShortVersionString $$VERSION" ./dist/macos/Impactor.app/Contents/Info.plist; \
		/usr/libexec/PlistBuddy -c "Set :CFBundleVersion $$VERSION" ./dist/macos/Impactor.app/Contents/Info.plist
	@mv ./dist/macos/plumeimpactor ./dist/macos/Impactor.app/Contents/MacOS/plumeimpactor
	@codesign --sign - --timestamp --options runtime ./dist/macos/Impactor.app
	@ditto -c -k --sequesterRsrc --keepParent ./dist/macos/Impactor.app ./dist/Impactor-macos-universal.zip
	@create-dmg --volname "Impactor" \
		--background "package/macos/background.png" \
		--window-pos 200 120 \
		--window-size 510 350 \
		--icon-size 100 \
		--icon Impactor.app 160 155 \
		--app-drop-link 360 155 \
		./dist/Impactor-macos-universal.dmg ./dist/macos/

linux:
	@cargo build --workspace --release --target x86_64-unknown-linux-gnu
	@cargo build --workspace --release --target aarch64-unknown-linux-gnu
	@cp ./target/x86_64-unknown-linux-gnu/release/plumeimpactor ./dist/plumeimpactor-linux-x86_64
	@cp ./target/x86_64-unknown-linux-gnu/release/plumesign ./dist/plumesign-linux-x86_64
	@cp ./target/aarch64-unknown-linux-gnu/release/plumeimpactor ./dist/plumeimpactor-linux-arm64
	@cp ./target/aarch64-unknown-linux-gnu/release/plumesign ./dist/plumesign-linux-arm64
	@strip ./dist/plumeimpactor-linux-x86_64
	@strip ./dist/plumesign-linux-x86_64
	@strip ./dist/plumeimpactor-linux-arm64
	@strip ./dist/plumesign-linux-arm64

windows:
	@cargo build --workspace --release --target x86_64-pc-windows-gnu
	@cp ./target/x86_64-pc-windows-gnu/release/plumeimpactor.exe ./dist/plumeimpactor-windows-x86_64.exe
	@cp ./target/x86_64-pc-windows-gnu/release/plumesign.exe ./dist/plumesign-windows-x86_64.exe
