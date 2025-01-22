#!/usr/bin/env bash

# WARN: assume PWD is project root
# WARN: This should not be run out of github CI

set -ex

cd app/js/
npm install && npm run build

cd ../../


# explicitly ignore things we want to for the app/ dir
rm .gitignore
{ echo "app/.packages/lets-talk-ui.fifthtry.site/.packages/";
  echo "app/.packages/lets-talk-ui.fifthtry.site/.fastn/";
  echo "app/js/node_modules/";
  echo "app/js/build.js";
  echo "app/js/build.js.map"; } >> .gitignore


cd app/
echo "uploading with following dir contents:"
ls -la
fastn upload lets-talk
