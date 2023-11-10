mod api;
mod controller;

#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![api::rgb_get])
}
