STATIC_LIB_NAME := libplume_core.a
TARGET_DIR := ../../target

gen:
	cargo build --features vendored-botan
	cargo run -p uniffi-bindgen generate --language swift --lib-file $(TARGET_DIR)/debug/libplume_core.dylib src/core.udl
