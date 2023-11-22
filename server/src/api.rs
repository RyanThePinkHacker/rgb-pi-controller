use std::sync::{Arc, Mutex};

use common::Color;
use rocket::{serde::json::Json, State};

use crate::controller::LightController;

pub struct ServerState {
    pub light_controller: LightController,
}

#[get("/rgb")]
pub fn rgb_get(state: &State<Arc<Mutex<ServerState>>>) -> Json<Color> {
    let state_unlocked = state.lock().expect("Mutex lock failed.");
    let current_color = state_unlocked.light_controller.get_color().clone();
    Json(current_color)
}

#[get("/rgb/<hex>")]
pub fn rgb_post(state: &State<Arc<Mutex<ServerState>>>, hex: &str) {
    state
        .lock()
        .expect("Mutex lock failed.")
        .light_controller
        .set_color(Color::from_hex_string(hex).expect(&format!("Invalid hex color: {}.", hex)))
        .expect("Failed to set color.");
}
