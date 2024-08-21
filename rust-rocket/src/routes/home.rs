#[get("/")]
pub fn index() -> &'static str {
    "Welcome to the home page!"
}
