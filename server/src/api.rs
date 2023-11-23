use std::sync::{Arc, Mutex};

use common::{Color, PWM};
use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    response::content::RawHtml,
    serde::json::Json,
    Request, Response, State,
};

use crate::controller::LightController;

pub struct ServerState {
    pub light_controller: LightController,
}

type StatePointer<'a> = &'a State<Arc<Mutex<ServerState>>>;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Cross-Origin-Resource-Sharing Middleware",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("access-control-allow-origin", "*"));
        response.set_header(Header::new(
            "access-control-allow-methods",
            "GET, PATCH, OPTIONS",
        ));
    }
}

#[get("/")]
pub fn index() -> RawHtml<&'static str> {
    RawHtml(include_str!("../../resources/index.html"))
}

#[get("/rgb")]
pub fn rgb_get(state: StatePointer) -> Json<Color> {
    let state_unlocked = state.lock().expect("Mutex lock failed.");
    let current_color = state_unlocked.light_controller.get_color().clone();
    Json(current_color)
}

#[post("/rgb/<hex>")]
pub fn rgb_post(state: StatePointer, hex: &str) {
    state
        .lock()
        .expect("Mutex lock failed.")
        .light_controller
        .set_color(Color::from_hex_string(hex).expect(&format!("Invalid hex color: {}.", hex)))
        .expect("Failed to set color.");
}

#[get("/pwm")]
pub fn pwm_get(state: StatePointer) -> Json<PWM> {
    let pwm = state
        .lock()
        .expect("Mutex lock failed.")
        .light_controller
        .get_pwm_frequency();
    Json(PWM::new(pwm))
}

#[post("/pwm/<frequency>")]
pub fn pwm_post(state: StatePointer, frequency: f64) {
    state
        .lock()
        .expect("Mutex lock failed.")
        .light_controller
        .set_pwm_frequency(frequency);
}
