#!/usr/bin/env bash

# WARN: assume PWD is project root

set -ex

function download_release() {
  repo_name=$1
  version=$2
  binary_name=$3

  url="https://github.com/${repo_name}/releases/download/${version}/${binary_name}"

  time curl -L -o "${binary_name}" "${url}"
}

download_release fifthtry-community/auth 0.1.2 email_auth_provider.wasm

mv email_auth_provider.wasm template/

# Build talk.wasm

cargo build --release --target wasm32-unknown-unknown
mv target/wasm32-unknown-unknown/release/talk.wasm template/

cd template/ && fastn upload lets-talk-template
