use crate::services;
use rocket_contrib::json::Json;
use services::quotes::Quote;

#[get("/quotes/themes")]
pub fn get_quotes_theme() -> Json<[String;1]> {
    Json( services::quotes::themes() )
}

#[get("/quotes/json?<theme>")]
pub fn get_json_quotes(theme: Option<String>) -> Json<Quote> {
    Json(services::quotes::get(theme))
}


#[get("/quotes/<platform>?<theme>")]
pub fn get_string_quotes(platform: String, theme: Option<String>) -> String {
    match platform.as_str() {
        "whatsapp" => services::quotes::whatsapp(theme),
        _ => format!("{} not found", platform)
    }
}
