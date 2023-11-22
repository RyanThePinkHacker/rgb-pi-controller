use std::sync::{Arc, Mutex};

use api::ServerState;
use controller::LightController;
use rppal::gpio::Gpio;

mod api;
mod controller;

#[macro_use]
extern crate rocket;

const RED_PIN: u8 = 17;
const GREEN_PIN: u8 = 27;
const BLUE_PIN: u8 = 22;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let gpio = Gpio::new()?;
    let light_controller = LightController::new(gpio, RED_PIN, GREEN_PIN, BLUE_PIN)?;
    rocket::build()
        .mount("/", routes![api::rgb_get, api::rgb_post])
        .manage(Arc::new(Mutex::new(ServerState { light_controller })))
        .launch()
        .await?;
    Ok(())
}
