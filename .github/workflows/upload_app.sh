#!/usr/bin/env bash

# WARN: assume PWD is project root

set -ex

cd app/js/
npm install && npm run build

cd ..

fastn upload lets-talk
