#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "my Blog"
}

#[launch]
fn rocket() -> _ {
    println("launched");
    rocket::build().mount("/", routes![index])
}
