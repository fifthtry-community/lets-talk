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
    pushd2 "${PROJ_ROOT}/lets-talk.fifthtry.site" || return 1
    echo "Using $FASTN to serve lets-talk.fifthtry.site/"
    $FASTN serve --offline
    popd2
}

function run-www() {
    pushd2 "${PROJ_ROOT}/lets-talk.fifthtry-community.com" || return 1
    echo "Using $FASTN to serve lets-talk.fifthtry-community.com/"
    $FASTN serve --offline
    popd2
}

function update-ui() {
    pushd2 "${PROJ_ROOT}/lets-talk.fifthtry.site" || return 1
    $FASTN update
    popd2
}

function update-www() {
    pushd2 "${PROJ_ROOT}/lets-talk.fifthtry-community.com" || return 1
    $FASTN update
    popd2
}

function update-template() {
    pushd2 "${PROJ_ROOT}/lets-talk-template.fifthtry.site" || return 1
    $FASTN update
    popd2
}

function build-wasm() {
    pushd2 "${PROJ_ROOT}" || return 1
    # this script should be used both for local development and for building on ci
    sh scripts/build-wasm.sh
    popd2
}

function run-template() {
    pushd2 "${PROJ_ROOT}/lets-talk-template.fifthtry.site" || return 1

    build-wasm
    $FASTN --trace serve --offline
}
