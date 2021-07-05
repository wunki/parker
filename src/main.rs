#[macro_use]
extern crate rocket;

/// This is where we explain how to make use of our
/// service. Best is to guide the user towards their
/// first step.
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// This is where we will indicate if our service
/// is healthy. Check connection to external systems,
/// like the database.
#[get("/health_check")]
fn health_check() -> &'static str {
    todo!()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, health_check])
}
