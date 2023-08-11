use rocket::*;

#[get("/")]
fn index() -> String {
    "Hello Rocket".to_owned()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
}
