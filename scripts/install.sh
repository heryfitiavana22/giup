#!/bin/bash

set -e

VERSION="v0.1.0"
REPO="heryfitiavana22/giup"

detect_os() {
    case "$(uname -s)" in
        Linux*)     OS="x86_64-unknown-linux-gnu" ;;
        Darwin*)    OS="x86_64-apple-darwin" ;;
        *)          echo "Unsupported OS"; exit 1 ;;
    esac
}

install_bin() {
    FILENAME="giup-$OS.tar.gz"
    URL="https://github.com/$REPO/releases/download/$VERSION/$FILENAME"
    TMP_DIR=$(mktemp -d)

    echo "Downloading $URL..."
    curl -L "$URL" -o "$TMP_DIR/$FILENAME"

    echo "Extracting..."
    tar -xzf "$TMP_DIR/$FILENAME" -C "$TMP_DIR"

    echo "Installing to /usr/local/bin (sudo required)"
    sudo mv "$TMP_DIR/giup" /usr/local/bin/
    sudo chmod +x /usr/local/bin/giup

    echo "giup installed"
    rm -rf "$TMP_DIR"
}

detect_os
install_bin