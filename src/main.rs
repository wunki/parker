#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {
    parker::rocket()
}
