

let hmm = 1;
function addBookmark() {
    browser.bookmarks.create(
        {
            title: "blar" + hmm
        }
    );

    hmm++;
};

browser.browserAction.onClicked.addListener(addBookmark);

var gettingBookmarks = browser.bookmarks.get("Steam");
gettingBookmarks.then(onFulfilled, onRejected);
gettingBookmarks();

function onFulfilled(bookmarks) {
    console.log(bookmarks);
};

function onRejected(error) {
    console.log(`An error: ${error}`);
};
