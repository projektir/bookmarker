# bookmarker

### Database Setup

```
sudo -u postgres psql
create database bookmarkdb;
create user admin_user with encrypted password 'dummy';
grant all privileges on database bookmarkdb to admin_user;
```

```
diesel setup --database-url postgres://admin_user:dummy@localhost/bookmarkdb

\c bookmarkdb
```

Development:
```
diesel migration generate create_bookmarks --database-url postgres://admin_user:dummy@localhost/bookmarkdb
diesel migration revert --database-url postgres://admin_user:dummy@localhost/bookmarkdb
diesel migration redo --database-url postgres://admin_user:dummy@localhost/bookmarkdb
diesel migration run --database-url postgres://admin_user:dummy@localhost/bookmarkdb
```

#### Initial Test Data

```
INSERT INTO bookmarks (id, title, url, icon) VALUES
    ('kMXD6KBOkYFR', 'Habitica', 'https://habitica.com/', 'WIP'),
    ('Louf03RhfFx2', 'Rust', 'https://www.rust-lang.org/', 'WIP'),
    ('hoBdEMMzukcg', 'RBE', 'https://doc.rust-lang.org/rust-by-example/', 'WIP'),
    ('M3R3XgYCzoGM', 'Wowhead Classic', 'https://classic.wowhead.com/', 'WIP');

INSERT INTO tags (name) VALUES
    ('Rust'), -- 1
    ('Programming'); -- 2
INSERT INTO tags (name, description) VALUES ('WoW', 'World of Warcraft'); -- 3

INSERT INTO bookmark_tags (bookmark_id, tag_id) VALUES
    ('Louf03RhfFx2', 1),
    ('Louf03RhfFx2', 2),
    ('hoBdEMMzukcg', 1),
    ('hoBdEMMzukcg', 2),
    ('M3R3XgYCzoGM', 3);
```

### GraphQL Queries

http://localhost:3030/playground

```
query {
  bookmarks {
    id
    title
    url
    icon
    notes
    tags
    relevant
    created
  }
}
```

```
curl 'http://127.0.0.1:3030/graphql' \
  -H 'Content-Type: application/json' \
  -d '{"query": "query { bookmarks { id title url } }"}'
```

### Extension Development

```
npm install --global web-ext
cd extension/bookmarker
web-ext run
```

Use `about::debugging` to properly debug the script.

To test, enable the Bookmarks Toolbar and drag Other Bookmarks onto it. Click the pin icon. You
will see some plain text in the popup, and you can click on it. "add bookmark" will put
generated folders into Other Bookmarks, and "print tree" will give you a log of the current
bookmark tree.

For easier testing of real, persistent bookmarks, the script will also search for a folder
named "Test Bookmarks" on the Bookmarks Toolbar, which can be maintained on the full firefox
and then the extension can be loaded by hand there.

For console settings, choose "Persist Logs" so it doesn't refresh and eat the logs all the time.

### Notes

It should be noted that a few things are hardcoded right now, like server URL or literal text
by which some actions are picked up, so if things suddenly stop working, the reason might be
trivial.

### License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Icons from https://www.iconfinder.com/iconsets/bitsies.
