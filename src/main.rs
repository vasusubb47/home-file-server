use rocket::{
    serde::json::{serde_json::json, Value},
    *,
};

#[get("/")]
fn index() -> Value {
    json!({
        "hello": "world",
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/api/", routes![index])
}
