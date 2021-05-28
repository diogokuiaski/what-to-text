use crate::services;

#[get("/birthday/<platform>?<name>")]
pub fn birthday(platform: String, name: Option<String>) -> String {
    match platform.as_str() {
        "whatsapp" => services::whatsapp::birthday(name),
        _ => format!("{} not found", platform)
    }
}