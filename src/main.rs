// src/main.rs

// dependencies
use rocket::{get, routes, Build, Rocket};
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

const DAY_1_SET_LIST: [&str; 17] = ["Creeping Death", "Harvester of Sorrow", "Leper Messiah", "King Nothing", "72 Seasons", "If Darkness Had a Son", "Orca Attack (Rob and Kirk jam)", "The Day That Never Comes", "Shadows Follow", "Orion", "Kirk Solo", "Nothing Else Matters", "Sad But True", "Blackened", "Fuel", "Seek and Destroy", "Master of Puppets"];

const DAY_2_SET_LIST: [&str; 16] = ["Whiplash", "For Whom the Bell Tolls", "Ride the Lightning", "The Memory Remains", "Lux Aeterna", "Screaming Suicide", "Rob and Kirk jam (Sonics)", "Welcome Home (Sanitarium)", "Wherever I May Roam", "The Call of Cthulu", "The Unforgiven", "Inamorata", "Fight Fire with Fire", "Moth into Flame", "One", "Enter Sandman"];

// function which returns a 200 OK response with empty body
#[get("/health")]
fn health() -> Status {
    Status::Ok
}

// function which returns the index page template
#[get("/")]
fn index() -> Template {
    let set_lists = (DAY_1_SET_LIST, DAY_2_SET_LIST);
    Template::render("index", context! { set_lists })
}

// function to create a rocket instance
fn create() -> Rocket<Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/api", routes![health])
        .mount("/static", FileServer::from(relative!("static")))
        .mount("/assets", FileServer::from(relative!("assets")))
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = create();

    Ok(rocket.into())
}
