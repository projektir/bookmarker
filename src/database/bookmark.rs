use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::sql_types::Text;

use super::{DatabaseError, PooledConnection};

#[derive(Queryable, Debug)]
pub struct Bookmark {
    pub id: String,
    pub title: String,
    pub url: String,
    pub icon: String,
    pub notes: String,
    pub relevant: DateTime<Utc>,
    pub created: DateTime<Utc>,
}

#[derive(QueryableByName, Debug)]
pub struct TagName {
    #[sql_type = "Text"]
    pub name: String,
}

#[derive(Queryable, Debug)]
pub struct Tag {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub color: String,
}

pub fn query_bookmarks(conn: &PooledConnection) -> Result<Vec<Bookmark>, DatabaseError> {
    use super::schema::bookmarks::dsl::*;

    Ok(bookmarks.load::<Bookmark>(conn)?)
}

pub fn bookmark_tags(
    conn: &PooledConnection,
    bookmark_id: &str,
) -> Result<Vec<TagName>, DatabaseError> {
    use diesel::sql_query;

    Ok(sql_query(
        r#"
SELECT tags.name
FROM bookmark_tags
INNER JOIN tags ON bookmark_tags.tag_id = tags.id
WHERE bookmark_tags.bookmark_id = $1;
"#,
    )
    .bind::<Text, _>(bookmark_id)
    .load(conn)?)
}
