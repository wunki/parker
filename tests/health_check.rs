use rocket::http::{ContentType, Status};
use rocket::local::blocking::Client;

#[test]
fn health_returns_200_ok() {
    let client = Client::tracked(parker::rocket()).expect("valid rocket instance");
    let response = client.get("/health").dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.content_type(), Some(ContentType::JSON))
}
