#[get("/hello/<name>")]
pub fn hello(name: String) -> String {
    format!("Hello, {}! :)", name.as_str())
}