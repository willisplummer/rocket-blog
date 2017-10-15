use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

use self::schema::posts;
use self::schema::posts::dsl::posts as all_posts;

mod schema {
  infer_schema!("env:DATABASE_URL");
}

#[table_name = "posts"]
#[derive(Serialize, Queryable, Insertable, Debug, Clone)]
pub struct Post {
  pub id: Option<i32>,
  pub title: String,
  pub body: String,
}

#[derive(FromForm)]
pub struct Entry {
  pub title: String,
  pub body: String,
}

impl Post {
  pub fn all(conn: &SqliteConnection) -> Vec<Post> {
    all_posts
      .order(posts::id.desc())
      .load::<Post>(conn)
      .unwrap()
  }

  pub fn insert(entry: Entry, conn: &SqliteConnection) -> bool {
    let p = Post {
      id: None,
      title: entry.title,
      body: entry.body,
    };
    diesel::insert(&p).into(posts::table).execute(conn).is_ok()
  }

  pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
    diesel::delete(all_posts.find(id)).execute(conn).is_ok()
  }
}
