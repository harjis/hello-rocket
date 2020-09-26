#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate diesel;

use hello_rocket::establish_connection;
use hello_rocket::schema::hellos::dsl::hellos;
use diesel::{RunQueryDsl, QueryDsl};
use hello_rocket::models::Hello;

#[get("/")]
fn index() -> &'static str {
    let connection = establish_connection();
    let results = hellos
        .load::<Hello>(&connection)
        .expect("error Loading");
    println!("Displaying {} posts", results.len());
    for hello in results {
        println!("{}: {}", hello.id, hello.hello);
    }
    "Hello World!!"
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
