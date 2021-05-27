#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;

fn main() {
    rocket::ignite().mount("/", routes![controllers::hello]).launch();
}
