function makeIndent(indentLength) {
    return "— ".repeat(indentLength);
}

function logItems(bookmarkItem, indent) {
    let bookmarkTitle = bookmarkItem.title;
    if (!bookmarkItem.parentId) {
        bookmarkTitle = "ROOT";
    }

    if (bookmarkItem.type === "bookmark") {
        console.log(`${makeIndent(indent)}${bookmarkItem.index} ${bookmarkTitle} ` +
            `[${bookmarkItem.url}] (${bookmarkItem.type})`);
    } else {
        if (bookmarkItem.type === "separator") {
            bookmarkTitle = makeIndent(5);
        }

        console.log(`${makeIndent(indent)}${bookmarkItem.index} ${bookmarkTitle} ` +
            `(${bookmarkItem.type})`);
    }
    
    if (bookmarkItem.type === "folder") {
        indent++;
    }

    if (bookmarkItem.children) {
        for (child of bookmarkItem.children) {
            logItems(child, indent);
        }
    }

    indent--;
}

function logTree(bookmarkItems) {
    logItems(bookmarkItems[0], 0);
}

function onRejected(error) {
    console.log(`An error: ${error}`);
}

let generatedCount = 1;
function addBookmark() {
    if (generatedCount < 4) {
        browser.bookmarks.create(
            {
                title: "Generated " + generatedCount
            }
        );
    }

    generatedCount++;

    var gettingTree = browser.bookmarks.getTree();
    gettingTree.then(logTree, onRejected);
};

browser.browserAction.onClicked.addListener(addBookmark);
