#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket_contrib;

use rocket_contrib::serve::StaticFiles;

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", StaticFiles::from("static"))
}

fn main() {
    rocket().launch();
}