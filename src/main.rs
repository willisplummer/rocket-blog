#![feature(plugin, decl_macro, custom_derive, const_fn)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

mod post;
mod db;

use rocket::Rocket;

use rocket_contrib::Json;

use post::{Post};

#[derive(Debug, Serialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
    posts: Vec<Post>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context {
            msg: msg,
            posts: Post::all(conn),
        }
    }
}

#[get("/")]
fn index(conn: db::Conn) -> Json<Vec<Post>> { 
    Json(
        Context::raw(&conn, None).posts
    )
}

fn rocket() -> (Rocket, Option<db::Conn>) {
    let pool = db::init_pool();
    let conn = if cfg!(test) {
        Some(db::Conn(
            pool.get().expect("database connection for testing"),
        ))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/", routes![index]);

    (rocket, conn)
}

fn main() {
    rocket().0.launch();
}
