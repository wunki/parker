use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn index_returns_200_ok() {
    let client = Client::tracked(parker::rocket()).expect("valid rocket instance");
    let response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::Ok);
}
