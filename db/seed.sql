CREATE TABLE posts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title VARCHAR NOT NULL,
    body VARCHAR NOT NULL,
);
INSERT INTO posts
    (title, body)
VALUES
    ("demo post", "demo post body");
