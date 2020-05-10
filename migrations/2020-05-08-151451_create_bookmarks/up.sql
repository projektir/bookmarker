CREATE TABLE bookmarks (
  id UUID PRIMARY KEY,
  title TEXT NOT NULL,
  url TEXT NOT NULL, -- fine if it's defunct, just have a url
  icon TEXT NOT NULL, -- file path or url
  notes TEXT NOT NULL default '',
  relevant timestamptz default now() NOT NULL, -- date of when the original content was created
  created timestamptz default now() NOT NULL
);

CREATE TABLE tags (
  id BIGSERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL default '',
  color TEXT NOT NULL default '#FFFFFF' -- outer color, text color is the inverse of this color
);

CREATE TABLE bookmark_tags (
  id BIGSERIAL PRIMARY KEY,
  bookmark_id UUID NOT NULL,
  tag_id BIGINT NOT NULL,

  FOREIGN KEY(bookmark_id) REFERENCES bookmarks(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
);

CREATE INDEX idx_bookmark_tags__bookmark_id ON bookmark_tags(bookmark_id);
CREATE INDEX idx_bookmark_tags__tag_id ON bookmark_tags(tag_id);
