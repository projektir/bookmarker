table! {
    bookmarks (id) {
        id -> Text,
        title -> Text,
        url -> Text,
        icon -> Text,
        notes -> Text,
        relevant -> Timestamptz,
        created -> Timestamptz,
    }
}

table! {
    bookmark_tags (id) {
        id -> Int8,
        bookmark_id -> Text,
        tag_id -> Int8,
    }
}

table! {
    tags (id) {
        id -> Int8,
        name -> Text,
        description -> Text,
        color -> Text,
    }
}

table! {
    units (id) {
        id -> Int4,
        name -> Text,
        building -> Text,
        minerals -> Int4,
        gas -> Int4,
        supply -> Int4,
    }
}

joinable!(bookmark_tags -> bookmarks (bookmark_id));
joinable!(bookmark_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    bookmarks,
    bookmark_tags,
    tags,
    units,
);
