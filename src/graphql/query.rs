use juniper::FieldResult;

use crate::database::bookmark::{bookmark_tags, query_bookmarks};

use super::schema::{Bookmark, Context};

pub struct Query;

#[juniper::graphql_object(Context = Context)]
impl Query {
    async fn bookmarks(context: &Context) -> FieldResult<Vec<Bookmark>> {
        let connection = context.db.get()?;

        let bookmarks = query_bookmarks(&connection)?;

        bookmarks
            .into_iter()
            .map(|bookmark| {
                let tags = bookmark_tags(&connection, bookmark.id)?
                    .into_iter()
                    .map(|tagname| tagname.name)
                    .collect();

                Ok(Bookmark {
                    id: bookmark.id,
                    title: bookmark.title,
                    url: bookmark.url,
                    icon: bookmark.icon,
                    notes: bookmark.notes,
                    tags,
                    relevant: bookmark.relevant,
                    created: bookmark.created,
                })
            })
            .collect::<Result<Vec<_>, _>>()
    }
}
