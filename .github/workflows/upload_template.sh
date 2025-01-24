#!/usr/bin/env bash

# WARN: assume PWD is project root
# WARN: This should not be run out of github CI

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

# explicitly ignore things we want to for the template/ dir
{ echo "template/.packages/";
  echo "template/.fastn/"; } > .gitignore

cd template/
echo "uploading with following dir contents:"
ls -la
fastn upload lets-talk-template
