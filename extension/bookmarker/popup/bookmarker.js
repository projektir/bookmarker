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
        for (child of bookmarkItem.children) {
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
    } else if (chosenAction === "send request") {
        sendRequest();
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

function sendRequest() {
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
    })
    .catch((error) => {
        console.error("Error:", error);
    });
}
