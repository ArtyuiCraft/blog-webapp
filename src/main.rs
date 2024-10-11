#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "my Bloga"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
