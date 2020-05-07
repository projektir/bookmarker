CREATE TABLE bookmarks (
  id UUID PRIMARY KEY,
  title TEXT NOT NULL,
  url TEXT NOT NULL, -- fine if it's defunct, just have a url
  icon TEXT NOT NULL, -- file path or url
  notes TEXT NOT NULL default '',
  -- to timestamp or to timestamptz, that is the question
  relevant timestamp default (now() at time zone 'utc') NOT NULL,
  created timestamp default (now() at time zone 'utc') NOT NULL
);

CREATE TABLE tags (
  id SERIAL PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL default '',
  color TEXT NOT NULL default '#FFFFFF' -- outer color, text color is the inverse of this color
);

CREATE TABLE bookmark_tags (
  id SERIAL PRIMARY KEY,
  bookmark_id UUID NOT NULL,
  tag_id INTEGER NOT NULL,

  FOREIGN KEY(bookmark_id) REFERENCES bookmarks(id),
  FOREIGN KEY(tag_id) REFERENCES tags(id)
);

CREATE INDEX idx_bookmark_tags__bookmark_id ON bookmark_tags(bookmark_id);
CREATE INDEX idx_bookmark_tags__tag_id ON bookmark_tags(tag_id);
