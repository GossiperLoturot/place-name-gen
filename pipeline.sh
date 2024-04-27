#!/bin/sh

FILENAME=$(basename $0)

# move to the directory of the script
PRJ_DIR=$(cd $(dirname $0) && pwd)
echo "[$FILENAME] Running pipeline in $PRJ_DIR"

# check if the required tools are installed
if type cargo &> /dev/null; then
    echo "[$FILENAME] cargo is installed"
else
    echo "[$FILENAME] cargo is not installed"
    exit 1
fi

if type wasm-pack &> /dev/null; then
    echo "[$FILENAME] wasm-pack is installed"
else
    echo "[$FILENAME] wasm-pack is not installed"
    exit 1
fi

if type bun &> /dev/null; then
    echo "[$FILENAME] bun is installed"
else
    echo "[$FILENAME] bun is not installed"
    exit 1
fi

# run the pipeline
echo "[$FILENAME] Warm up database"
START_TIME=$(date +%s)
cd $PRJ_DIR/warmup
cargo run --release
END_TIME=$(date +%s)
echo "[$FILENAME] Done warmimp up database ($(($END_TIME - $START_TIME)) secs)"

echo "[$FILENAME] Generate native lib"
START_TIME=$(date +%s)
cd $PRJ_DIR/gen-native
cargo build --release
END_TIME=$(date +%s)
echo "[$FILENAME] Done generating native lib ($(($END_TIME - $START_TIME)) secs)"

echo "[$FILENAME] Generate wasm lib"
START_TIME=$(date +%s)
cd $PRJ_DIR/gen-wasm
RUSTFLAGS=
RUSTC_WRAPPER=
wasm-pack build --release --target web
END_TIME=$(date +%s)
echo "[$FILENAME] Done generating wasm lib ($(($END_TIME - $START_TIME)) secs)"

echo "[$FILENAME] Build and run demo-wasm"
START_TIME=$(date +%s)
cd $PRJ_DIR/demo-wasm
bun install
bun run build
bun run serve
END_TIME=$(date +%s)
echo "[$FILENAME] Done building and running $(($END_TIME - $START_TIME)) seconds"
