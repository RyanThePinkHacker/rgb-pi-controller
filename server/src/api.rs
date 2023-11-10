use common::Color;
use rocket::{serde::json::Json, State};

use crate::controller::LightController;

pub struct ServerState {
    pub light_controller: LightController,
}

#[get("/rgb")]
pub fn rgb_get(state: &State<ServerState>) -> Json<&Color> {
    Json(state.light_controller.get_color())
}
