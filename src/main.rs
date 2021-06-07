#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod controllers;
mod services;

fn main() {
    rocket::ignite().mount("/", routes![
        controllers::birthday::birthday,
        controllers::quotes::get_string_quotes,
        controllers::quotes::get_json_quotes,
        controllers::quotes::get_quotes_theme,

    ]).launch();
}
