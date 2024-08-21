pub mod home;
pub mod user;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![home::index, user::get_user]
}
