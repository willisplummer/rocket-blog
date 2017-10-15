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
#[cfg(test)]
mod tests;

use rocket::Rocket;
use rocket::request::{FlashMessage, Form};
use rocket::response::{Flash, Redirect};
use rocket_contrib::Template;

use post::{Entry, Post};

#[derive(Debug, Serialize)]
struct Context<'a, 'b> {
    msg: Option<(&'a str, &'b str)>,
    posts: Vec<Post>,
}

impl<'a, 'b> Context<'a, 'b> {
    pub fn err(conn: &db::Conn, msg: &'a str) -> Context<'static, 'a> {
        Context {
            msg: Some(("error", msg)),
            posts: Post::all(conn),
        }
    }

    pub fn raw(conn: &db::Conn, msg: Option<(&'a str, &'b str)>) -> Context<'a, 'b> {
        Context {
            msg: msg,
            posts: Post::all(conn),
        }
    }
}

#[post("/", data = "<entry_form>")]
fn new(entry_form: Form<Entry>, conn: db::Conn) -> Flash<Redirect> {
    let entry = entry_form.into_inner();
    if entry.title.is_empty() {
        Flash::error(Redirect::to("/"), "Title cannot be empty.")
    } else if entry.body.is_empty() {
        Flash::error(Redirect::to("/"), "Body cannot be empty.")
    } else if Post::insert(entry, &conn) {
        Flash::success(Redirect::to("/"), "Post successfully added.")
    } else {
        Flash::error(Redirect::to("/"), "Whoops! The server failed.")
    }
}

#[delete("/<id>")]
fn delete(id: i32, conn: db::Conn) -> Result<Flash<Redirect>, Template> {
    if Post::delete_with_id(id, &conn) {
        Ok(Flash::success(Redirect::to("/"), "Entry was deleted."))
    } else {
        Err(Template::render(
            "index",
            &Context::err(&conn, "Couldn't delete post."),
        ))
    }
}

#[get("/")]
fn index(msg: Option<FlashMessage>, conn: db::Conn) -> Template {
    Template::render(
        "index",
        &match msg {
            Some(ref msg) => Context::raw(&conn, Some((msg.name(), msg.msg()))),
            None => Context::raw(&conn, None),
        },
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
        .mount("/", routes![index])
        .mount("/entry/", routes![new, delete])
        .attach(Template::fairing());

    (rocket, conn)
}

fn main() {
    rocket().0.launch();
}
