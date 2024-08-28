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
    Template::render("index", context! { message: "Metallica Rules!" })
}

// function to create a rocket instance
fn create() -> Rocket<Build> {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/api", routes![health])
        .mount("/static", FileServer::from(relative!("static")))
}

// main function
#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = create();

    Ok(rocket.into())
}
