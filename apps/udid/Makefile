CC = clang
CFLAGS = -O2 -framework Foundation -fobjc-arc -mmacosx-version-min=11.0

SRC = main.m
INC = gestalt.h

OUT_DIR = build
OUT = $(OUT_DIR)/udid

SDK = $(shell xcrun --sdk macosx --show-sdk-path)

LDFLAGS = -lmobilegestalt -isysroot $(SDK)

all: $(OUT)

$(OUT): $(SRC) $(INC)
	mkdir -p $(OUT_DIR)
	$(CC) $(CFLAGS) $(LDFLAGS) -o $(OUT) $(SRC)

clean:
	rm -rf $(OUT_DIR)
