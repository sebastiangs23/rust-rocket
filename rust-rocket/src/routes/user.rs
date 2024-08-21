#[get("/user/<name>")]
pub fn get_user(name: String) -> String {
    format!("Hello, {}!", name)
}
