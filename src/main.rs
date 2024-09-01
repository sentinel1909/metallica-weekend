// src/main.rs

// dependencies
use rocket::{get, routes, Build, Rocket};
use rocket::fs::{relative, FileServer};
use rocket::http::Status;
use rocket_dyn_templates::{context, Template};

// function which returns a 200 OK response with empty body
#[get("/health")]
fn health() -> Status {
    Status::Ok
}

// function which returns the index page template
#[get("/")]
fn index() -> Template {
    let set_list = ["Creeping Death", "Harvester of Sorrow", "Leper Messiah", "King Nothing", "72 Seasons", "If Darkness Had a Son", "Orca Attack (Rob and Kirk jam)", "The Day That Never Comes", "Shadows Follow", "Orion", "Kirk Solo", "Nothing Else Matters", "Sad But True", "Blackened", "Fuel", "Seek and Destroy", "Master of Puppets"];
    Template::render("index", context! { set_list })
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
