/// This is where we explain how to make use of our
/// service. Best is to guide the user towards their
/// first step.
#[get("/")]
pub fn get_welcome() -> &'static str {
    "Hello, world!"
}

