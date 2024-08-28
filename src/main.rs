// src/main.rs

// dependencies
use rocket::{Build, get, Rocket, routes};
use rocket_dyn_templates::{context, Template};
use rocket::fs::{relative, FileServer};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { message: "Metallica Rules!" })
}

// function to create a rocket instance
fn create() -> Rocket<Build> {
    rocket::build().attach(Template::fairing()).mount("/", routes![index]).mount("/static", FileServer::from(relative!("static")))
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = create();

    Ok(rocket.into())
}
