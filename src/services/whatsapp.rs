pub fn birthday(name: Option<String>) -> String {
    match name {
        Some(x) => format!("Happy birthday, {}!", x),
        None => format!("Happy birthday!")
    }
}