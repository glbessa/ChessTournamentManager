#[macro_use] extern crate rocket;

use rocket::response::content;
use rocket::http::ContentType;

#[get("/")]
fn index() -> (ContentType, &'static str) {
    (ContentType::HTML, "<h1>Hello world!</h1>")
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}

