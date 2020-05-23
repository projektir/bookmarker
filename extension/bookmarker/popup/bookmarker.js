
"use strict";

let bookmarks = new Map;
let parent;

function makeIndent(indentLength) {
    return "— ".repeat(indentLength);
}

function logItems(bookmarkItem, indent) {
    let bookmarkTitle = bookmarkItem.title;
    if (!bookmarkItem.parentId) {
        bookmarkTitle = "ROOT";
    }

    if (bookmarkItem.type === "bookmark") {
        console.log(`${makeIndent(indent)}${bookmarkItem.index} ${bookmarkItem.id} ${bookmarkTitle} ` +
            `[${bookmarkItem.url}] (${bookmarkItem.type})`);
    } else {
        if (bookmarkItem.type === "separator") {
            bookmarkTitle = makeIndent(5);
        }

        console.log(`${makeIndent(indent)}${bookmarkItem.index} ${bookmarkItem.id} ${bookmarkTitle} ` +
            `(${bookmarkItem.type})`);
    }
    
    if (bookmarkItem.type === "folder") {
        indent++;
    }

    if (bookmarkItem.children) {
        for (let child of bookmarkItem.children) {
            logItems(child, indent);
        }
    }

    indent--;
}

function logTree(bookmarkItems) {
    // This should always exist, right?
    var bookmarksToolbar = bookmarkItems[0].children.find(function(child) {
        return child.title === "Bookmarks Toolbar";
    });

    var testBookmarks = bookmarksToolbar.children.find(function(child) {
        return child.title === "Test Bookmarks";
    });

    if (testBookmarks) {
        logItems(testBookmarks, 0);
    } else {
        logItems(bookmarkItems[0], 0);
    }
}

function onRejected(error) {
    console.log(`An error: ${error}`);
}

document.addEventListener("click", function(e) {
    if (!e.target.classList.contains("action")) {
        return;
    }

    var chosenAction = e.target.textContent;
    if (chosenAction === "print tree") {
        printTree();
    } else if (chosenAction === "add bookmark") {
        addBookmark();
    } else if (chosenAction === "convert") {
        convert();
    } else if (chosenAction === "sync") {
        sync();
    }
});

function printTree() {
    var gettingTree = browser.bookmarks.getTree();

    gettingTree.then(logTree, onRejected);
};

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
};

// Convert bookmarks to a map
function convert() {
    var gettingTree = browser.bookmarks.getTree();
    gettingTree.then(convertTree, onRejected);
}

function convertTree(bookmarkItems) {
    // This should always exist, right?
    var bookmarksToolbar = bookmarkItems[0].children.find(function(child) {
        parent = child; // Don't worry about it
        return child.title === "Bookmarks Toolbar";
    });

    var testBookmarks = bookmarksToolbar.children.find(function(child) {
        parent = child; // Don't worry about it
        return child.title === "Test Bookmarks";
    });

    if (testBookmarks) {
        convertItems(testBookmarks, 0);
    } else {
        convertItems(bookmarkItems[0], 0);
    }

    console.log("Bookmarks: " + bookmarks);
    for (let item in bookmarks) {
        console.log(item);
    }
}

function convertItems(bookmarkItem, indent) {
    bookmarks[bookmarkItem.id] = bookmarkItem;

    if (bookmarkItem.type === "folder") {
        indent++;
    }

    if (bookmarkItem.children) {
        for (let child of bookmarkItem.children) {
            convertItems(child, indent);
        }
    }

    indent--;
}

// Call convert first
function sync() {
    // Stupidest sync ever for testing purposes

    fetch("http://127.0.0.1:3030/graphql/", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify({"query": "query { bookmarks { id title url } }"}),
    })
    .then(response => response.json())
    .then(data => {
        console.log("Success:", data);

        for (let item of data.data.bookmarks) { // Why is data.data still a thing
            console.log("item: " + item);
            if (!bookmarks[item.id]) {
                let newBookmark;

                if (item.url) { // bookmark
                    newBookmark = browser.bookmarks.create({
                        title: item.title,
                        url: item.url,
                        parentId: parent.id
                    });
                } else if (!item.title) { // separator
                    newBookmark = browser.bookmarks.create({
                        type: "separator",
                        parentId: parent.id
                    });
                } else { // folder
                    newBookmark = browser.bookmarks.create({
                        title: item.title,
                        parentId: parent.id
                    });
                }

                bookmarks[item.id] = newBookmark;
            }
        }

        console.log("Bookmarks: " + bookmarks);
        for (let item in bookmarks) {
            console.log(item);
        }
    })
    .catch((error) => {
        console.error("Error:", error);
    });
}
