#!/usr/bin/env just --justfile
# just manual: https://github.com/casey/just/#readme

_default:
    @just --list

# Env variables.
export TRUNK_CONFIG := "./crates/frontend/Trunk.toml"

# Variables.
BUILD_DIR := "./build"

# Build backend and frontend for release.
build-release:
    #!/bin/bash

    BUILD_DIR={{BUILD_DIR}}
    STATIC_DIR=$BUILD_DIR/static

    # Create new final directory.
    rm -rf $BUILD_DIR
    mkdir -p $BUILD_DIR/static $BUILD_DIR/logs

    # Build backend and static files.
    cargo build --profile non-wasm-release --bin backend
    cargo build --features ssr --profile non-wasm-release --bin frontend
    trunk build --release --features ssr ./crates/frontend/trunk_index.html --dist $STATIC_DIR --public-url /static/

    # Remove assets directory on the static folder (used when using CSR).
    rm -r $STATIC_DIR/assets

    # Copy necessary files to final directory.
    cp -r ./assets $BUILD_DIR
    rm $BUILD_DIR/assets/justfile
    cp -r ./configs $BUILD_DIR
    cp -f ./target/non-wasm-release/backend $BUILD_DIR/backend
    cp -f ./target/non-wasm-release/frontend $BUILD_DIR/frontend

    # Optimize static files
    find $STATIC_DIR/*.wasm -exec cp {} ./target/unoptimized.wasm \; -exec wasm-snip --snip-rust-panicking-code {} -o {} \; -exec wasm-opt -Oz {} -o {} \;
    find $STATIC_DIR/*.js -exec npx terser {} -c -m --output {} \;
    find $STATIC_DIR/snippets/**/*.js -exec npx terser {} -c -m --output {} \;
    find $STATIC_DIR/*.css -exec npx csso {} --comments none --output {} \;
    # npx critical --b test -c tailwind-base*.css -w 320 -h 480 $STATIC_DIR/index.html -i > $STATIC_DIR/index.html

    echo "Compress wasm:"
    npx brotli-cli compress -q 11 --glob --bail false $STATIC_DIR/*.wasm || true
    echo "Compress js and snippets js:"
    npx brotli-cli compress -q 11 --glob --bail false $STATIC_DIR/*.js || true
    npx brotli-cli compress -q 11 --glob --bail false $STATIC_DIR/snippets/**/*.js || true
    echo "Compress css:"
    npx brotli-cli compress -q 11 --glob --bail false $STATIC_DIR/*.css || true

    echo "Build finished."

# Cleans the project.
clean:
    rm -rf {{BUILD_DIR}} ./node_modules ./crates/frontend/styles/dist
    cargo clean
    trunk clean

# Cleans the logs.
clean-logs:
    rm -f ./logs/*

# Runs clippy on the sources.
check:
    cargo clippy --locked -- -D warnings

# Builds and opens documentation in-browser without the dependencies docs.
docs:
    cargo doc --open --no-deps

# Builds and opens documentation in-browser with the dependencies docs.
docs-deps:
    cargo doc --open

# Expands macro in file and outputs it to console.
expand-macro FILE:
    rustc +nightly -Zunpretty=expanded {{FILE}}

# Cargo and clippy fix.
fix:
    cargo clippy --fix --allow-dirty --allow-staged

# Format code.
format:
    just rustfmt
    just _format_tailwindcss

# Make .githooks this project hooks lookup directory.
init-git-hooks:
    git config --local core.hooksPath .githooks

# Install needed dev dependencies and configurations.
install-init-dev:
    just init-git-hooks
    npm install
    cargo install --force dioxus-cli
    cargo install trunk
    cargo install sd
    just install-mold-linker
    just install-udeps
    rustup target add wasm32-unknown-unknown

# Install mold linker for faster compilation linker.
install-mold-linker:
    rm -rf mold
    git clone https://github.com/rui314/mold.git
    mkdir ./mold/build
    sudo ./mold/install-build-deps.sh
    cmake -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_COMPILER=c++ ./mold/ -B ./mold/build
    cmake --build ./mold/build/ -j $(nproc)
    sudo cmake --install ./mold/build/
    rm -rf mold

# Install cargo udeps.
install-udeps:
    cargo install cargo-udeps --locked

# Check npm packages for updates.
npm-check-updates:
    npx npm-check-updates -u

# Serve frontend.
dioxus-serve-csr PORT="5555" BACKEND_PORT="5550":
    cd {{justfile_directory()}}/crates/frontend; dioxus serve --features csr --port {{PORT}} --hot-reload

# Run frontend ssr.
run-frontend-ssr PORT="5556" STATIC_DIR="./target/static" ASSETS_DIR="./assets" DEBUG_FILTER="info" OPTION="":
    # Stop process using same port.
    fuser -k {{PORT}}/tcp || true
    FRONTEND_GENERAL_RUN_ENV=development cargo run --bin frontend {{OPTION}} -- --port {{PORT}} -s {{STATIC_DIR}} --assets-dir {{ASSETS_DIR}} -l {{DEBUG_FILTER}}


# Run backend.
run-backend PORT="5555" FRONTEND_ADDR="127.0.0.1" FRONTEND_PORT="5556" DEBUG_FILTER="info" OPTION="":
    # Stop process using same port.
    fuser -k {{PORT}}/tcp || true
    BACKEND_GENERAL_RUN_ENV=development cargo run --bin backend {{OPTION}} -- --port {{PORT}} -l {{DEBUG_FILTER}} --frontend-addr={{FRONTEND_ADDR}} --frontend-port={{FRONTEND_PORT}}

# Format using custom rustfmt.
rustfmt:
    find -type f -path "./crates/*" -path "*.rs" | xargs ./rustfmt --edition 2021

# Runs all tests.
test-all:
    cargo test --locked

# Runs macros of the specified package.
test PACKAGE:
    cargo test -p {{PACKAGE}} --locked

# Use udeps to find unused dependencies.
udeps:
    cargo +nightly udeps --all-targets

_grep_toml_config FILE GROUP_ENV CONFIG_VAR:
    grep -A 100 "^\[{{GROUP_ENV}}\]" {{FILE}} | grep -m 1 -oP '^{{CONFIG_VAR}}\s?=\s?"?\K[^"?]+'

_format_tailwindcss:
    #!/bin/bash
    FILES=$(find -type f -path "./crates/frontend/*" -path "*.rs" | xargs grep -il -E 'rsx!\s?\(') && \

    # Cycle through each file that contains an html! macro.
    for FILE in $FILES; do
        echo "Formatting $FILE"

        # Get each class="..." present.
        CLASSES=$(grep -oE 'class:\s?"[^"|(.)]*"' $FILE)

        IFS=$'\n' # make newlines the only separator, needs to be reset.

        # Cycle through each class.
        for CLASS in $CLASSES; do

            CLASS=$(echo $CLASS | sed -E 's/class:\s?/class=/g')

            # Prettify class.
            CLASS_PRETTY=$(echo "<img $CLASS>" | npx prettier --plugin prettier-plugin-tailwindcss --parser html --bracket-same-line true --print-width 1000000)

            # Remove extra tag and transform back to class: "...".
            CLASS_FINAL=$(echo $CLASS_PRETTY | grep -oE 'class="[^"|(.)]*"' | sed "s/class=/class: /g")

            # Replace in files.
            sd -s $CLASS $CLASS_FINAL $FILE
        done

        unset IFS
    done
