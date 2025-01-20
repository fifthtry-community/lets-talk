# shellcheck disable=SC2155
export PROJ_ROOT=$(pwd)
FASTN=${FASTN_BINARY:-fastn}

function pushd2() {
    PUSHED=$(pwd)
    cd "${PROJDIR}""$1" >> /dev/null || return
}

function popd2() {
    cd "${PUSHED:-$PROJDIR}" >> /dev/null || return
    unset PUSHED
}

function run-ui() {
  pushd2 "${PROJ_ROOT}/app/.packages/lets-talk-ui.fifthtry.site" || return 1

  echo "Using $FASTN to serve lets-talk-ui.fifthtry.site/"

  $FASTN serve --port 8002 --offline

  popd2
}
