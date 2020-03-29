#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate nalgebra;

mod collide;
mod targeting;
mod velocity;

use targeting::{find_target, Field, Roid};

#[cfg(test)] mod tests;

// use rocket::State;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize, Debug)]
struct GameState {
    field: Field,
    firing_position: (f32, f32),
    bullet_speed: f32,
    roids: Vec<Roid>
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/", format = "json", data = "<game_state>")]
fn update(game_state: Json<GameState>) -> JsonValue {
    find_target(
        &nalgebra::Point2::<f32>::new(game_state.firing_position.0, game_state.firing_position.1),
        game_state.bullet_speed,
        &game_state.field,
        &game_state.roids,
    );
    json!({
        "status": "coolio...",
    })
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/update", routes![update])
        .register(catchers![not_found])
}

fn main() {
    rocket().launch();
}
