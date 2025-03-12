function getMainAreaWidth() {
    // FIXME: This does not work. This function is called before ftd.column with #main-area is rendered so it can't find it :(
    // return document.getElementById("main-area").clientWidth;
    return 1000;
}

function getMainAreaHeight() {
    return 193;
}
