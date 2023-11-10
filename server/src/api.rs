use common::Color;
use rocket::serde::json::Json;

#[get("/rgb")]
pub fn rgb_get() -> Json<Color> {
    Json(Color::from_rgb(255, 255, 255))
}
