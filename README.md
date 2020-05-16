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
diesel migration generate create_bookmarks --database-url postgres://admin_user:dummy@localhost/bookmarkdb

\c bookmarkdb
```

#### Initial Test Data

```
INSERT INTO bookmarks (id, title, url, icon) VALUES
    ('0d725b11-d160-4b62-a94f-fcc8cdd09bed', 'Habitica', 'https://habitica.com/', 'WIP'),
    ('bcecdd37-ccfe-4c45-ac90-8b5ab101e1b3', 'Rust', 'https://www.rust-lang.org/', 'WIP'),
    ('4c9f154f-0405-4e8e-847d-0f660f887caa', 'RBE', 'https://doc.rust-lang.org/rust-by-example/', 'WIP'),
    ('c5bcecc6-4678-4dc3-95f2-f0c2c8280ef0', 'Wowhead Classic', 'https://classic.wowhead.com/', 'WIP');

INSERT INTO tags (name) VALUES
    ('Rust'), -- 1
    ('Programming'); -- 2
INSERT INTO tags (name, description) VALUES ('WoW', 'World of Warcraft'); -- 3

INSERT INTO bookmark_tags (bookmark_id, tag_id) VALUES
    ('bcecdd37-ccfe-4c45-ac90-8b5ab101e1b3', 1),
    ('bcecdd37-ccfe-4c45-ac90-8b5ab101e1b3', 2),
    ('4c9f154f-0405-4e8e-847d-0f660f887caa', 1),
    ('4c9f154f-0405-4e8e-847d-0f660f887caa', 2),
    ('c5bcecc6-4678-4dc3-95f2-f0c2c8280ef0', 3);
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

### Extension Development

```
npm install --global web-ext
cd extension/bookmarker
web-ext run
```

To test, enable the Bookmarks Toolbar and drag Other Bookmarks onto it. Click the green puzzle icon, and 
the extension will add a silly folder to it.

Use `about::debugging` to properly debug the script.

### License

This project is licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
